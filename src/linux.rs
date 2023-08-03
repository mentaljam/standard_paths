use std::{
    collections::HashMap,
    env, fs,
    io::{BufRead, BufReader, Error, ErrorKind},
    os::{linux::fs::MetadataExt, unix::fs::PermissionsExt},
    path::PathBuf,
};

use crate::{LocationType, StandardPaths};

macro_rules! get_var_or_home {
    ($var_name:expr, $($sub_dirs:expr),*) => {
        match env::var($var_name) {
            Ok(path) => PathBuf::from(path),
            _ => match home::home_dir() {
                Some(mut path) => {
                    $(
                        path.push($sub_dirs);
                    )*
                    path
                },
                _ => return Err(StandardPaths::home_dir_err())
            }
        }
    }
}

fn xdg_config_dirs() -> Vec<PathBuf> {
    // http://standards.freedesktop.org/basedir-spec/latest/
    match env::var("XDG_CONFIG_DIRS") {
        Ok(paths) => {
            let mut paths: Vec<PathBuf> = paths.split(':').map(PathBuf::from).collect();
            paths.dedup();
            paths
        }
        _ => vec!["/etc/xdg".into()],
    }
}

fn xdg_data_dirs() -> Vec<PathBuf> {
    // http://standards.freedesktop.org/basedir-spec/latest/
    match env::var("XDG_DATA_DIRS") {
        Ok(paths) => {
            let mut paths: Vec<PathBuf> = paths.split(':').map(PathBuf::from).collect();
            paths.dedup();
            let mut res = Vec::new();
            for path in paths {
                if path.is_dir() {
                    let path_str = path.to_str().unwrap();
                    if !path_str.is_empty() && path_str.starts_with('/') {
                        res.push(path.canonicalize().unwrap());
                    }
                }
            }
            res
        }
        _ => {
            vec!["/usr/local/share".into(), "/usr/share".into()]
        }
    }
}

impl StandardPaths {
    #[inline]
    pub(super) fn writable_location_impl(&self, location: LocationType) -> Result<PathBuf, Error> {
        match location {
            LocationType::HomeLocation => home::home_dir().ok_or_else(StandardPaths::home_dir_err),
            LocationType::TempLocation => Ok(env::temp_dir()),
            LocationType::AppCacheLocation | LocationType::GenericCacheLocation => {
                // http://standards.freedesktop.org/basedir-spec/basedir-spec-0.6.html
                let mut path = get_var_or_home!("XDG_CACHE_HOME", ".cache");
                if location == LocationType::AppCacheLocation {
                    self.append_organization_and_app(&mut path);
                }
                Ok(path)
            }

            LocationType::AppDataLocation
            | LocationType::AppLocalDataLocation
            | LocationType::GenericDataLocation => {
                let mut path = get_var_or_home!("XDG_DATA_HOME", ".local", "share");
                if location == LocationType::AppDataLocation
                    || location == LocationType::AppLocalDataLocation
                {
                    self.append_organization_and_app(&mut path);
                }
                Ok(path)
            }

            LocationType::ConfigLocation
            | LocationType::GenericConfigLocation
            | LocationType::AppConfigLocation => {
                // http://standards.freedesktop.org/basedir-spec/latest/
                let mut path = get_var_or_home!("XDG_CONFIG_HOME", ".config");
                if location == LocationType::AppConfigLocation {
                    self.append_organization_and_app(&mut path);
                }
                Ok(path)
            }

            LocationType::RuntimeLocation => {
                // http://standards.freedesktop.org/basedir-spec/latest/
                let uid = nix::unistd::geteuid();
                let user_id = uid.as_raw();
                let (path, md) = match env::var("XDG_RUNTIME_DIR") {
                    Ok(path) => {
                        let md = fs::metadata(&path)?;
                        if !md.is_dir() {
                            return Err(Error::new(
                                ErrorKind::Other,
                                format!(
                                    "'XDG_RUNTIME_DIR' points to '{path}' which is not a directory"
                                ),
                            ));
                        }
                        (PathBuf::from(path), md)
                    }
                    _ => {
                        let user = match nix::unistd::User::from_uid(uid) {
                            Ok(opt) => opt.unwrap(),
                            Err(err) => return Err(Error::new(
                                ErrorKind::NotFound,
                                format!("Failed to detect current user: {err}"),
                            ))
                        };
                        let mut runtime_dir = String::from("runtime-");
                        runtime_dir.push_str(&user.name);
                        let mut path = env::temp_dir();
                        path.push(runtime_dir);
                        let md = fs::metadata(&path)?;
                        if !md.is_dir() {
                            fs::create_dir_all(&path)?;
                        }
                        (path, md)
                    }
                };

                // The directory MUST be owned by the user
                let ts_uid = md.st_uid();
                if ts_uid != user_id {
                    return Err(Error::new(
                        ErrorKind::PermissionDenied,
                        format!(
                            "Wrong ownership on runtime directory '{}' - {ts_uid} instead of {user_id}",
                            path.to_string_lossy(),
                        ),
                    ));
                }
                // And its Unix access mode MUST be 0700.
                let mut permissions = md.permissions();
                if permissions.mode() != 0o40700 {
                    permissions.set_mode(0o40700);
                }

                Ok(path)
            }

            LocationType::FontsLocation | LocationType::ApplicationsLocation => {
                let dir = if location == LocationType::FontsLocation {
                    "fonts"
                } else {
                    "applications"
                };
                let mut path = self.writable_location_impl(LocationType::GenericDataLocation)?;
                path.push(dir);
                Ok(path)
            }

            _ => {
                // http://www.freedesktop.org/wiki/Software/xdg-user-dirs
                let mut config = get_var_or_home!("XDG_CONFIG_HOME", ".config");
                config.push("user-dirs.dirs");
                let file = fs::File::open(&config)?;
                let buf_reader = BufReader::new(file);
                let mut lines = HashMap::new();
                for line in buf_reader.lines() {
                    let line = line.unwrap();
                    if line.starts_with("XDG") {
                        let parts = line.split('=').collect::<Vec<&str>>();
                        let key = parts[0];
                        let key = parts[0].get(4..key.len() - 4).unwrap();
                        let mut value = parts[1];
                        if value.len() > 2 && value.starts_with('"') && value.ends_with('"') {
                            value = value.get(1..value.len() - 1).unwrap();
                        }
                        lines.insert(key.to_string(), value.to_string());
                    }
                }

                let key = match location {
                    LocationType::DesktopLocation => "DESKTOP",
                    LocationType::DocumentsLocation => "DOCUMENTS",
                    LocationType::PicturesLocation => "PICTURES",
                    LocationType::MusicLocation => "MUSIC",
                    LocationType::MoviesLocation => "VIDEOS",
                    LocationType::DownloadLocation => "DOWNLOAD",
                    _ => "",
                };
                if lines.contains_key(key) {
                    let value = &lines[key];
                    if value.starts_with("$HOME") {
                        let mut path = match home::home_dir() {
                            Some(path) => path,
                            _ => return Err(StandardPaths::home_dir_err()),
                        };
                        let value = value.get(6..).unwrap();
                        path.push(value);
                        return Ok(path);
                    }
                    return Ok(value.into());
                }

                let dir = match location {
                    LocationType::DesktopLocation => "Desktop",
                    LocationType::DocumentsLocation => "Documents",
                    LocationType::PicturesLocation => "Pictures",
                    LocationType::MusicLocation => "Music",
                    LocationType::MoviesLocation => "Videos",
                    LocationType::DownloadLocation => "Downloads",
                    _ => return Err(Error::new(ErrorKind::Other, "Unexpected error")),
                };
                let mut path = match home::home_dir() {
                    Some(path) => path,
                    _ => return Err(StandardPaths::home_dir_err()),
                };
                path.push(dir);
                Ok(path)
            }
        }
    }

    #[inline]
    pub(super) fn standard_locations_impl(
        &self,
        location: LocationType,
    ) -> Result<Vec<PathBuf>, Error> {
        let mut res: Vec<PathBuf> = match location {
            LocationType::ConfigLocation | LocationType::GenericConfigLocation => xdg_config_dirs(),
            LocationType::AppConfigLocation => {
                let mut dirs = xdg_config_dirs();
                for dir in dirs.iter_mut() {
                    self.append_organization_and_app(dir);
                }
                dirs
            }

            LocationType::GenericDataLocation => xdg_data_dirs(),

            LocationType::ApplicationsLocation => {
                let mut dirs = xdg_data_dirs();
                for dir in dirs.iter_mut() {
                    dir.push("applications");
                }
                dirs
            }

            LocationType::AppDataLocation | LocationType::AppLocalDataLocation => {
                let mut dirs = xdg_data_dirs();
                for dir in dirs.iter_mut() {
                    self.append_organization_and_app(dir);
                }
                dirs
            }

            LocationType::FontsLocation => match home::home_dir() {
                Some(mut path) => {
                    path.push(".fonts");
                    vec![path]
                }
                _ => return Err(StandardPaths::home_dir_err()),
            },

            _ => Vec::new(),
        };

        let path = self.writable_location_impl(location)?;
        res.insert(0, path);

        Ok(res)
    }
}

/// Detect if `path` is an executable based on its rights
fn is_executable<P>(path: P) -> bool
where
    P: Into<PathBuf>,
{
    let path = path.into();
    match fs::metadata(path) {
        Ok(md) => md.permissions().mode() & 0o111 != 0,
        _ => false,
    }
}

const EXTENSIONS: [&str; 3] = ["bin", "run", "sh"];

#[inline]
pub(super) fn find_executable_in_paths_impl<S, P>(name: S, paths: P) -> Option<Vec<PathBuf>>
where
    S: Into<String>,
    P: AsRef<Vec<PathBuf>>,
{
    let name = name.into();
    let path = PathBuf::from(&name);

    // Check absolute paths
    if path.is_absolute() && is_executable(&name) {
        return Some(vec![path]);
    }

    // Check paths
    let mut paths = paths.as_ref().clone();
    paths.retain(|p| !p.to_str().unwrap().is_empty() && p.is_dir());
    let paths = {
        let mut paths2 = Vec::new();
        for path in paths {
            let path = path.canonicalize().unwrap();
            if !paths2.contains(&path) {
                paths2.push(path);
            }
        }
        paths2
    };

    // At first search the provided name
    let mut res = Vec::new();
    for mut path in paths.iter().cloned() {
        path.push(&name);
        if is_executable(&path) {
            res.push(path);
        }
    }

    // Then check if an extension could be appended
    if path.extension().is_none() {
        for mut path in paths.iter().cloned() {
            path.push(&name);
            for ext in &EXTENSIONS {
                let mut full_path = path.clone();
                full_path.set_extension(ext);
                if is_executable(&full_path) {
                    res.push(full_path);
                }
            }
        }
    }

    if res.is_empty() {
        None
    } else {
        Some(res)
    }
}

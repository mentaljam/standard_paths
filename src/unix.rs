extern crate users;

use std::path::PathBuf;
use std::env;
use self::users::{
    get_user_by_uid,
    get_current_uid
};
use std::fs;
use std::os::linux::fs::MetadataExt;
use std::os::unix::fs::PermissionsExt;
use std::io::prelude::*;
use std::io::BufReader;
use std::collections::HashMap;

use ::LocationType;
use ::StandardLocation;


macro_rules! get_var_or_home {
    ($var_name:expr, $($sub_dirs:expr),*) => {
        match env::var($var_name) {
            Ok(path) => PathBuf::from(path),
            _ => match env::home_dir() {
                Some(mut path) => {
                    $(
                        path.push($sub_dirs);
                    )*
                    path
                },
                _ => return None
            }
        };
    }
}

impl StandardLocation {
    fn append_organization_and_app(&self, path: &mut PathBuf) {
        if !self.organisation_name.is_empty() {
            path.push(&self.organisation_name);
        }
        if !self.app_name.is_empty() {
            path.push(&self.app_name);
        }
    }

    #[inline]
    #[doc(hidden)]
    pub fn writable_location_impl(&self, location: LocationType) -> Option<PathBuf> {

        match location {
            LocationType::HomeLocation => env::home_dir(),
            LocationType::TempLocation => Some(env::temp_dir()),
            LocationType::CacheLocation | LocationType::GenericCacheLocation => {
                // http://standards.freedesktop.org/basedir-spec/basedir-spec-0.6.html
                let mut path = get_var_or_home!("XDG_CACHE_HOME", ".cache");
                if location == LocationType::CacheLocation {
                    self.append_organization_and_app(&mut path);
                }
                Some(path)
            },

            LocationType::AppDataLocation | 
            LocationType::AppLocalDataLocation | 
            LocationType::GenericDataLocation => {
                let mut path = get_var_or_home!("XDG_DATA_HOME", ".local", "share");
                if location == LocationType::AppDataLocation ||
                   location == LocationType::AppLocalDataLocation {
                    self.append_organization_and_app(&mut path);
                }
                Some(path)
            },

            LocationType::ConfigLocation |
            LocationType::GenericConfigLocation |
            LocationType::AppConfigLocation => {
                // http://standards.freedesktop.org/basedir-spec/latest/
                let mut path = get_var_or_home!("XDG_CONFIG_HOME", ".config");
                if location == LocationType::AppConfigLocation {
                    self.append_organization_and_app(&mut path);
                }
                Some(path)
            },

            LocationType::RuntimeLocation => {
                // http://standards.freedesktop.org/basedir-spec/latest/
                let user = get_user_by_uid(get_current_uid()).unwrap();
                let (path, md) = match env::var("XDG_RUNTIME_DIR") {
                    Ok(path) => {
                        let md = match fs::metadata(&path) {
                            Ok(md) => md,
                            Err(err) => {
                                eprintln!("Couldn't open '{}' from XDG_RUNTIME_DIR: {}", path, err);
                                return None;
                            }
                        };
                        if !md.is_dir() {
                            eprintln!("XDG_RUNTIME_DIR points to '{}' which is not a directory", path);
                            return None;
                        }
                        (PathBuf::from(path), md)
                    }
                    _ => {
                        let mut runtime_dir = String::from("runtime-");
                        runtime_dir.push_str(user.name());
                        let mut path = env::temp_dir();
                        path.push(runtime_dir);
                        let md = match fs::metadata(&path) {
                            Ok(md) => {
                                if !md.is_dir() {
                                    match fs::create_dir_all(&path) {
                                        Err(err) => {
                                            eprintln!("Error creating runtime directory {:?}: {}",
                                                      path, err);
                                            return None;
                                        },
                                        _ => ()
                                    }
                                }
                                md
                            },
                            Err(err) => {
                                eprintln!("Couldn't open runtime directory {:?}: {}", path, err);
                                return None;
                            }
                        };
                        println!("XDG_RUNTIME_DIR not set, defaulting to {:?}", path);
                        (PathBuf::from(path), md)
                    }
                };

                // The directory MUST be owned by the user
                if md.st_uid() != user.uid() {
                    eprintln!("Wrong ownership on runtime directory {:?}, {} instead of {}",
                              path, md.st_uid(), user.uid());
                    return None;
                }
                // And its Unix access mode MUST be 0700.
                let mut permissions = md.permissions();
                if permissions.mode() != 0o40700 {
                    permissions.set_mode(0o40700);
                }

                Some(path)
            },

            LocationType::FontsLocation | LocationType::ApplicationsLocation => {
                let dir = if location == LocationType::FontsLocation {
                    "fonts"
                } else {
                    "applications"
                };
                let mut path = match self.writable_location_impl(LocationType::GenericDataLocation) {
                    Some(path) => path,
                    _ => return None
                };
                path.push(dir);
                Some(path)
            },
            
            _ => {
                // http://www.freedesktop.org/wiki/Software/xdg-user-dirs
                let mut config = get_var_or_home!("XDG_CONFIG_HOME", ".config");
                config.push("user-dirs.dirs");
                let file = match fs::File::open(&config) {
                    Ok(file) => file,
                    Err(err) => {
                        eprintln!("Couldn't open {:?}: {}", config, err);
                        return None
                    }
                };
                let buf_reader = BufReader::new(file);
                let mut lines = HashMap::new();
                for line in buf_reader.lines() {
                    let line = line.unwrap();
                    if line.starts_with("XDG") {
                        let parts = line.split('=').collect::<Vec<&str>>();
                        let key = parts[0];
                        let key = parts[0].get(4..key.len() - 4).unwrap();
                        let mut value = parts[1];
                        if value.len() > 2 &&
                           value.starts_with('"') &&
                           value.ends_with('"') {
                               value = value.get(1..value.len() - 1).unwrap();
                           }
                        lines.insert(key.to_string(), value.to_string());
                    }
                }

                let key = match location {
                    LocationType::DesktopLocation   => "DESKTOP",
                    LocationType::DocumentsLocation => "DOCUMENTS",
                    LocationType::PicturesLocation  => "PICTURES",
                    LocationType::MusicLocation     => "MUSIC",
                    LocationType::MoviesLocation    => "VIDEOS",
                    LocationType::DownloadLocation  => "DOWNLOAD",
                    _ => ""
                };
                if lines.contains_key(key) {
                    let value = &lines[key];
                    if value.starts_with("$HOME") {
                        match env::home_dir() {
                            Some(mut path) => {
                                let value = value.get(6..).unwrap();
                                path.push(value);
                                return Some(path);
                            },
                            _ => return None
                        }
                    }
                    return Some(value.into());
                }

                let dir = match location {
                    LocationType::DesktopLocation       => "Desktop",
                    LocationType::DocumentsLocation     => "Documents",
                    LocationType::PicturesLocation      => "Pictures",
                    LocationType::MusicLocation         => "Music",
                    LocationType::MoviesLocation        => "Videos",
                    LocationType::DownloadLocation      => "Downloads",
                    _ => ""
                };
                if dir.is_empty() {
                    return None
                }
                let mut path = match env::home_dir() {
                    Some(path) => path,
                    _ => return None
                };
                path.push(dir);
                Some(path)
            }
        }
    }
}

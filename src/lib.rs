//! # Standard Paths
//!
//! A Rust library providing methods for accessing standard paths
//! on the local filesystem (config, cache, user directories and etc.).
//!
//! It's a port of [`QStandardPaths`](https://doc.qt.io/qt-5/qstandardpaths.html)
//! class of the Qt framework.
//!
//! ### Usage
//! ```
//! extern crate standard_paths;
//!
//! use standard_paths::*;
//! use standard_paths::LocationType::*;
//!
//! fn main() {
//!     let sp = StandardPaths::new("app", "org");
//!     println!("{:?}", sp.writable_location(AppLocalDataLocation));
//! }
//! ```

#![warn(missing_docs)]
#![warn(clippy::doc_markdown)]

#[cfg(target_os = "linux")]
mod linux;

#[cfg(target_os = "linux")]
use linux::*;

#[cfg(windows)]
mod windows;

#[cfg(windows)]
use windows::*;

use std::env;
use std::io::{Error, ErrorKind};
use std::path::{Path, PathBuf};

/// Constructs a new [`StandardPaths`] with the application name
/// derived from the `CARGO_PKG_NAME` variable.
///
/// ### Example
/// ```
/// use standard_paths::LocationType;
///
/// let sp = standard_paths::default_paths!();
/// println!("{:?}", sp.writable_location(LocationType::AppLocalDataLocation));
/// ```
#[macro_export]
macro_rules! default_paths {
    () => {
        $crate::StandardPaths::without_org(env!("CARGO_PKG_NAME"))
    };
}

/// Enumerates the standard location type.
///
/// Is used to call
/// [`StandardPaths::writable_location`] and
/// [`StandardPaths::find_executable_in_paths`].
///
/// Some of the values are used to acquire user-specific paths,
/// some are application-specific and some are system-wide.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LocationType {
    /// The user's home directory.
    ///
    /// * On Linux systems it's equal to the `$HOME` environment variable.
    /// * On the last Windows operating systems it's equal to the `%HomePath%`
    /// environment variable.
    HomeLocation,
    /// The user's desktop directory.
    DesktopLocation,
    /// The user's documents directory.
    DocumentsLocation,
    /// The directory for the user's downloaded files.
    ///
    /// This is a generic value. On Windows if no such directory exists,
    /// the directory for storing user documents is returned.
    DownloadLocation,
    /// The user's movies and videos directory.
    MoviesLocation,
    /// The user's music, recordings and other audio files directory.
    MusicLocation,
    /// The user's pictures, photos and screenshots directory.
    PicturesLocation,
    /// The user's applications directory.
    ///
    /// It might contain either executables, desktop files, or shortcuts.
    ///
    /// It's a platform-specific value.
    ApplicationsLocation,
    /// The user's fonts directory.
    FontsLocation,
    /// The directory for the runtime communication files (like Unix local sockets).
    ///
    /// This is a generic value. It could returns
    /// [`None`] on some systems.
    RuntimeLocation,
    /// A directory for storing temporary files.
    ///
    /// It might be application-specific, user-specific or system-wide.
    TempLocation,
    /// The directory for the persistent data shared across applications.
    ///
    /// This is a generic value.
    GenericDataLocation,
    /// The persistent application data directory.
    ///
    /// This is an application-specific directory.
    /// On the Windows operating system, this returns the roaming path.
    AppDataLocation,
    /// The local settings directory.
    ///
    /// This is a Windows-specific value.
    /// On all other platforms, it returns the same value as
    /// [`LocationType::AppDataLocation`].
    AppLocalDataLocation,
    /// The directory for the user-specific cached data shared across applications.
    ///
    /// This is a generic value. It could returns
    /// [`None`] from the appropriate methods if the system has no concept of shared cache.
    GenericCacheLocation,
    /// The user-specific cached data directory.
    ///
    /// This is an application-specific directory.
    AppCacheLocation,
    /// The user-specific configuration files directory.
    ///
    /// This may be either a generic value or application-specific.
    ConfigLocation,
    /// The user-specific configuration files directory.
    /// shared between multiple applications.
    ///
    /// This is a generic value.
    GenericConfigLocation,
    /// The user-specific configuration files directory.
    ///
    /// This is an application-specific value.
    AppConfigLocation,
}

/// Enumerates the locate option type.
///
/// Is used to call
/// [`StandardPaths::locate`] and
/// [`StandardPaths::locate_all`].
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LocateOption {
    /// Locate both files and directories (traversing symbolic links).
    LocateBoth,
    /// Locate only files.
    LocateFile,
    /// Locate only directories.
    LocateDirectory,
}

/// Stores application and organization names and provides all the crate methods.
pub struct StandardPaths {
    /// Application name.
    app_name: String,
    /// organization name.
    org_name: String,
}

impl StandardPaths {
    /// Constructs a new [`StandardPaths`] with the provided `app` and `org` names.
    pub fn new<S>(app: S, org: S) -> StandardPaths
    where
        S: Into<String>,
    {
        StandardPaths {
            app_name: app.into(),
            org_name: org.into(),
        }
    }

    /// Constructs a new [`StandardPaths`] with the provided `app` name and with an empty organization.
    pub fn without_org<S>(app: S) -> StandardPaths
    where
        S: Into<String>,
    {
        StandardPaths {
            app_name: app.into(),
            org_name: Default::default(),
        }
    }

    /// Append application suffix to the `path`.
    ///
    /// For example `~/.config` -> `~/.config/org/app`.
    ///
    /// # Arguments
    /// * `path` - a mutable [`PathBuf`] to which the app suffix should be appended.
    fn append_organization_and_app(&self, path: &mut PathBuf) {
        if !self.org_name.is_empty() {
            path.push(&self.org_name);
        }
        if !self.app_name.is_empty() {
            path.push(&self.app_name);
        }
    }

    /// Returns the directory where files of type `location` should be written to.
    ///
    /// Note: the returned path can be a directory that does not exist.
    ///
    /// Returns [`Error`] if the location cannot be determined.
    ///
    /// # Arguments
    /// * `location` - location type.
    pub fn writable_location(&self, location: LocationType) -> Result<PathBuf, Error> {
        self.writable_location_impl(location)
    }

    /// Returns all the directories of type `location`.
    ///
    /// The vector of locations is sorted by priority, starting with
    /// [self.writable location](struct.StandardPaths.html#method.writable_location)
    /// if it can be determined.
    ///
    /// Returns [`Error`] if the locations cannot be determined or
    /// an empty vector if no locations for the provided type are defined.
    ///
    /// # Arguments
    /// * `location` - location type.
    pub fn standard_locations(&self, location: LocationType) -> Result<Vec<PathBuf>, Error> {
        self.standard_locations_impl(location)
    }

    /// Returns the absolute file path to the executable with `name` in the system path.
    ///
    /// It also could be used to check a path to be an executable.
    ///
    /// Internally it calls the
    /// [`self.find_executable_in_paths`]
    /// method with the system path as the `paths` argument. On most operating systems
    /// the system path is determined by the `PATH` environment variable.
    ///
    /// Note: on Windows the executable extensions from the `PATHEXT` environment variable
    /// are automatically appended to the `name` if it doesn't contain any extension.
    ///
    /// Returns [`None`] if no executables are found or if the provided path is not executable.
    ///
    /// # Arguments
    /// * `name` - the name of the searched executable or an absolute path
    /// which should be checked to be executable.
    pub fn find_executable<S>(name: S) -> Option<Vec<PathBuf>>
    where
        S: Into<String>,
    {
        // Read system paths
        let path_var = match env::var("PATH") {
            Ok(var) => var,
            _ => return None,
        };
        let paths: Vec<PathBuf> = env::split_paths(&path_var).collect();
        StandardPaths::find_executable_in_paths(name, paths)
    }

    /// Returns the absolute file path to the executable with `name` in the provided `paths`.
    ///
    /// Note: on Windows the executable extensions from the `PATHEXT` environment variable
    /// are automatically appended to the `name` if it doesn't contain any extension.
    ///
    /// Returns [`None`] if no executables are found or if the provided path is not executable.
    ///
    /// # Arguments
    /// * `name` - the name of the searched executable or an absolute path
    /// which should be checked to be executable.
    /// * `paths` - the directories where to search for the executable.
    pub fn find_executable_in_paths<S, P>(name: S, paths: P) -> Option<Vec<PathBuf>>
    where
        S: Into<String>,
        P: AsRef<Vec<PathBuf>>,
    {
        find_executable_in_paths_impl(name, &paths)
    }

    /// Search for a file or directory called 'name' in the standard locations.
    ///
    /// Returns a full path to the first file or directory found.
    ///
    /// Returns [`Error`] if accessing the `location` failed or
    /// [`None`] if no such file or directory can be found.
    ///
    /// # Arguments
    /// * `location` - the location type where to search.
    /// * `name` - the name of the file or directory to search.
    /// * `option` - the type of entry to search.
    pub fn locate<P>(
        &self,
        location: LocationType,
        name: P,
        option: LocateOption,
    ) -> Result<Option<PathBuf>, Error>
    where
        P: AsRef<Path>,
    {
        let paths = self.standard_locations(location)?;
        for mut path in paths {
            path.push(&name);
            match option {
                LocateOption::LocateBoth => {
                    if path.exists() {
                        return Ok(Some(path));
                    }
                }
                LocateOption::LocateFile => {
                    if path.is_file() {
                        return Ok(Some(path));
                    }
                }
                LocateOption::LocateDirectory => {
                    if path.is_dir() {
                        return Ok(Some(path));
                    }
                }
            }
        }
        Ok(None)
    }

    /// Search for all files or directories called 'name' in the standard locations.
    ///
    /// Returns a vector of full paths to the all files or directories found.
    ///
    /// Returns [`Error`] if accessing the `location` failed or
    /// [`None`] if no such files or directories can be found.
    ///
    /// # Arguments
    /// * `location` - the location type where to search.
    /// * `name` - the name of the files or directories to search.
    /// * `option` - the type of entries to search.
    pub fn locate_all<P>(
        &self,
        location: LocationType,
        name: P,
        option: LocateOption,
    ) -> Result<Option<Vec<PathBuf>>, Error>
    where
        P: AsRef<Path>,
    {
        let paths = self.standard_locations(location)?;
        let mut res = Vec::new();
        for mut path in paths {
            path.push(&name);
            match option {
                LocateOption::LocateBoth => {
                    if path.exists() {
                        res.push(path);
                    }
                }
                LocateOption::LocateFile => {
                    if path.is_file() {
                        res.push(path);
                    }
                }
                LocateOption::LocateDirectory => {
                    if path.is_dir() {
                        res.push(path);
                    }
                }
            }
        }
        if res.is_empty() {
            Ok(None)
        } else {
            Ok(Some(res))
        }
    }

    #[inline]
    fn home_dir_err() -> Error {
        Error::new(ErrorKind::Other, "Error getting HOME directory")
    }
}

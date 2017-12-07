#[macro_use]
mod macros;

#[cfg(unix)]
mod unix;

#[cfg(unix)]
use unix::*;

#[cfg(windows)]
mod windows;

#[cfg(windows)]
use windows::*;

use std::env;
use std::path::PathBuf;


#[derive(Debug, Clone, PartialEq)]
pub enum LocationType {
    DesktopLocation,
    DocumentsLocation,
    FontsLocation,
    ApplicationsLocation,
    MusicLocation,
    MoviesLocation,
    PicturesLocation,
    TempLocation,
    HomeLocation,
    CacheLocation,
    GenericDataLocation,
    RuntimeLocation,
    ConfigLocation,
    DownloadLocation,
    GenericCacheLocation,
    GenericConfigLocation,
    AppDataLocation,
    AppConfigLocation,
    AppLocalDataLocation
}

pub struct StandardPaths {
    app_name: String,
    organisation_name: String
}

impl StandardPaths {
    pub fn new() -> StandardPaths {
        StandardPaths {
            app_name: match env::var("CARGO_PKG_NAME") {
                Ok(name) => name,
                _ => String::new()
            },
            organisation_name: String::new()
        }
    }

    pub fn new_with_names(app: &'static str, organisation: &'static str) -> StandardPaths {
        StandardPaths {
            app_name: app.into(),
            organisation_name: organisation.into()
        }
    }

    fn append_organization_and_app(&self, path: &mut PathBuf) {
        if !self.organisation_name.is_empty() {
            path.push(&self.organisation_name);
        }
        if !self.app_name.is_empty() {
            path.push(&self.app_name);
        }
    }

    pub fn writable_location(&self, location: LocationType) -> Option<PathBuf> {
        self.writable_location_impl(location)
    }

    pub fn standard_locations(&self, location: LocationType) -> Option<Vec<PathBuf>> {
        self.standard_locations_impl(location)
    }

    pub fn find_executable<S>(name: S) -> Option<Vec<PathBuf>>
    where S: Into<String> {
        let paths: Vec<PathBuf> = Vec::new();
        StandardPaths::find_executable_in_paths(name, paths)
    }

    pub fn find_executable_in_paths<S>(name: S, paths: Vec<PathBuf>) -> Option<Vec<PathBuf>>
    where S: Into<String> {
        find_executable_in_paths_impl(name, paths)
    }
}

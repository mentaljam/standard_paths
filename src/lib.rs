#[cfg(unix)]
mod unix;

#[cfg(windows)]
mod windows;

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
    DataLocation,
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

pub struct StandardLocation {
    app_name: String,
    organisation_name: String
}

impl StandardLocation {
    pub fn new() -> StandardLocation {
        StandardLocation {
            app_name: match env::var("CARGO_PKG_NAME") {
                Ok(name) => name,
                _ => String::new()
            },
            organisation_name: String::new()
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

    pub fn new_with_names(app: &'static str, organisation: &'static str) -> StandardLocation {
        StandardLocation {
            app_name: app.into(),
            organisation_name: organisation.into()
        }
    }

    pub fn writable_location(&self, location: LocationType) -> Option<PathBuf> {
        self.writable_location_impl(location)
    }

    pub fn standard_locations(&self, location: LocationType) -> Option<Vec<PathBuf>> {
        self.standard_locations_impl(location)
    }
}

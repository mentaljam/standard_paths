use std::path::PathBuf;
use std::env;

use ::LocationType;
use ::StandardLocation;


impl StandardLocation {
    pub fn writable_location_impl(&self, location: LocationType) -> Option<PathBuf> {
        assert!(false, "Unimplemented!");
        None
    }
}
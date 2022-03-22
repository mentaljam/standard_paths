extern crate winapi;

use std::env;
use std::ffi::{OsStr, OsString};
use std::io::{Error, ErrorKind};
use std::os::windows::ffi::{OsStrExt, OsStringExt};
use std::path::PathBuf;
use std::ptr;
use std::slice;

use self::winapi::shared::guiddef::GUID;
use self::winapi::shared::minwindef::DWORD;
use self::winapi::um::combaseapi::CoTaskMemFree;
use self::winapi::um::shlobj::SHGetKnownFolderPath;
use self::winapi::um::winbase::GetBinaryTypeW;
use self::winapi::um::winnt::PWSTR;

use LocationType;
use LocationType::*;
use StandardPaths;

/// https://msdn.microsoft.com/en-us/library/dd378457.aspx#FOLDERID_Desktop
#[allow(non_upper_case_globals)]
const FOLDERID_Desktop: GUID = GUID {
    Data1: 0xB4BFCC3A,
    Data2: 0xDB2C,
    Data3: 0x424C,
    Data4: [0xB0, 0x29, 0x7F, 0xE9, 0x9A, 0x87, 0xC6, 0x41],
};

/// https://msdn.microsoft.com/en-us/library/dd378457.aspx#FOLDERID_Documents
#[allow(non_upper_case_globals)]
const FOLDERID_Documents: GUID = GUID {
    Data1: 0xFDD39AD0,
    Data2: 0x238F,
    Data3: 0x46AF,
    Data4: [0xAD, 0xB4, 0x6C, 0x85, 0x48, 0x03, 0x69, 0xC7],
};

/// https://msdn.microsoft.com/en-us/library/dd378457.aspx#FOLDERID_Fonts
#[allow(non_upper_case_globals)]
const FOLDERID_Fonts: GUID = GUID {
    Data1: 0xFD228CB7,
    Data2: 0xAE11,
    Data3: 0x4AE3,
    Data4: [0x86, 0x4C, 0x16, 0xF3, 0x91, 0x0A, 0xB8, 0xFE],
};

/// https://msdn.microsoft.com/en-us/library/dd378457.aspx#FOLDERID_Programs
#[allow(non_upper_case_globals)]
const FOLDERID_Programs: GUID = GUID {
    Data1: 0xA77F5D77,
    Data2: 0x2E2B,
    Data3: 0x44C3,
    Data4: [0xA6, 0xA2, 0xAB, 0xA6, 0x01, 0x05, 0x4A, 0x51],
};

/// https://msdn.microsoft.com/en-us/library/dd378457.aspx#FOLDERID_Music
#[allow(non_upper_case_globals)]
const FOLDERID_Music: GUID = GUID {
    Data1: 0x4BD8D571,
    Data2: 0x6D19,
    Data3: 0x48D3,
    Data4: [0xBE, 0x97, 0x42, 0x22, 0x20, 0x08, 0x0E, 0x43],
};

/// https://msdn.microsoft.com/en-us/library/dd378457.aspx#FOLDERID_Videos
#[allow(non_upper_case_globals)]
const FOLDERID_Videos: GUID = GUID {
    Data1: 0x18989B1D,
    Data2: 0x99B5,
    Data3: 0x455B,
    Data4: [0x84, 0x1C, 0xAB, 0x7C, 0x74, 0xE4, 0xDD, 0xFC],
};

/// https://msdn.microsoft.com/en-us/library/dd378457.aspx#FOLDERID_Pictures
#[allow(non_upper_case_globals)]
const FOLDERID_Pictures: GUID = GUID {
    Data1: 0x33E28130,
    Data2: 0x4E1E,
    Data3: 0x4676,
    Data4: [0x83, 0x5A, 0x98, 0x39, 0x5C, 0x3B, 0xC3, 0xBB],
};

/// https://msdn.microsoft.com/en-us/library/dd378457.aspx#FOLDERID_Downloads
#[allow(non_upper_case_globals)]
const FOLDERID_Downloads: GUID = GUID {
    Data1: 0x374DE290,
    Data2: 0x123F,
    Data3: 0x4565,
    Data4: [0x91, 0x64, 0x39, 0xC4, 0x92, 0x5E, 0x46, 0x7B],
};

/// https://msdn.microsoft.com/en-us/library/dd378457.aspx#FOLDERID_LocalAppData
#[allow(non_upper_case_globals)]
const FOLDERID_LocalAppData: GUID = GUID {
    Data1: 0xF1B32785,
    Data2: 0x6FBA,
    Data3: 0x4FCF,
    Data4: [0x9D, 0x55, 0x7B, 0x8E, 0x7F, 0x15, 0x70, 0x91],
};

/// https://msdn.microsoft.com/en-us/library/dd378457.aspx#FOLDERID_RoamingAppData
#[allow(non_upper_case_globals)]
const FOLDERID_RoamingAppData: GUID = GUID {
    Data1: 0x3EB685DB,
    Data2: 0x65F9,
    Data3: 0x4CF6,
    Data4: [0xA0, 0x3A, 0xE3, 0xEF, 0x65, 0x72, 0x9F, 0x3D],
};

/// https://msdn.microsoft.com/en-us/library/dd378457.aspx#FOLDERID_ProgramData
#[allow(non_upper_case_globals)]
const FOLDERID_ProgramData: GUID = GUID {
    Data1: 0x62AB5D82,
    Data2: 0xFDC1,
    Data3: 0x4DC3,
    Data4: [0xA9, 0xDD, 0x07, 0x0D, 0x1D, 0x49, 0x5D, 0x97],
};

struct SafePwstr(PWSTR);

impl SafePwstr {
    pub fn new() -> Self {
        SafePwstr(ptr::null_mut())
    }
}

impl AsMut<PWSTR> for SafePwstr {
    fn as_mut(&mut self) -> &mut PWSTR {
        &mut self.0
    }
}

impl Into<PathBuf> for SafePwstr {
    fn into(self) -> PathBuf {
        unsafe {
            // Calculate length of wide C string
            let mut len = 0;
            while *self.0.offset(len) != 0 {
                len += 1;
            }
            // Convert to OsString
            let wpath: &[u16] = slice::from_raw_parts(self.0, len as usize);
            let path: OsString = OsStringExt::from_wide(wpath);
            // Return PathBuf
            PathBuf::from(path)
        }
    }
}

impl Drop for SafePwstr {
    /// Automatically free the string after leaving a scope.
    fn drop(&mut self) {
        unsafe { CoTaskMemFree(self.0 as *mut _) }
    }
}

macro_rules! sh_get_known_folder_path {
    ($id:ident, $var:pat, $then:block, $else:block) => {{
        unsafe {
            let mut raw_path = SafePwstr::new();
            // HRESULT::S_OK = 0
            let ok = SHGetKnownFolderPath(&$id, 0, ptr::null_mut(), raw_path.as_mut()) == 0;
            let $var: PathBuf = raw_path.into();
            if ok
                $then
            else
                $else
        }
    }};
}

impl StandardPaths {
    #[inline]
    #[doc(hidden)]
    pub fn writable_location_impl(&self, location: LocationType) -> Result<PathBuf, Error> {
        match location {
            DownloadLocation => {
                sh_get_known_folder_path!(FOLDERID_Downloads, path, { Ok(path) }, {
                    self.writable_location(DocumentsLocation)
                })
            }

            AppCacheLocation | GenericCacheLocation => {
                // FOLDERID_InternetCache points to IE's cache. Most applications seem to
                // be using a cache directory located in their AppData directory.
                let loc2 = if location == AppCacheLocation {
                    AppLocalDataLocation
                } else {
                    GenericDataLocation
                };
                let mut path = self.writable_location(loc2)?;
                path.push("cache");
                Ok(path)
            }

            RuntimeLocation | HomeLocation => env::home_dir().ok_or(StandardPaths::home_dir_err()),

            TempLocation => {
                let canonicalized = env::temp_dir().canonicalize().unwrap();
                Ok(PathBuf::from(
                    canonicalized.to_str().unwrap().get(4..).unwrap(),
                ))
            }

            _ => {
                let id = match location {
                    DesktopLocation => FOLDERID_Desktop,
                    DocumentsLocation => FOLDERID_Documents,
                    FontsLocation => FOLDERID_Fonts,
                    ApplicationsLocation => FOLDERID_Programs,
                    MusicLocation => FOLDERID_Music,
                    MoviesLocation => FOLDERID_Videos,
                    PicturesLocation => FOLDERID_Pictures,
                    AppLocalDataLocation
                    | GenericDataLocation
                    | ConfigLocation
                    | GenericConfigLocation
                    | AppConfigLocation => FOLDERID_LocalAppData,
                    AppDataLocation => FOLDERID_RoamingAppData,
                    _ => GUID {
                        Data1: 0x0,
                        Data2: 0x0,
                        Data3: 0x0,
                        Data4: [0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0],
                    },
                };
                sh_get_known_folder_path!(
                    id,
                    mut path,
                    {
                        if location == ConfigLocation
                            || location == AppConfigLocation
                            || location == AppDataLocation
                            || location == AppLocalDataLocation
                        {
                            self.append_organization_and_app(&mut path);
                        }
                        Ok(path)
                    },
                    { Err(Error::new(ErrorKind::Other, "Unexpected error")) }
                )
            }
        }
    }

    #[inline]
    #[doc(hidden)]
    pub fn standard_locations_impl(&self, location: LocationType) -> Result<Vec<PathBuf>, Error> {
        let mut dirs = Vec::new();
        let path = self.writable_location(location.clone())?;
        dirs.push(path);
        if location == ConfigLocation
            || location == AppConfigLocation
            || location == AppDataLocation
            || location == AppLocalDataLocation
            || location == GenericConfigLocation
            || location == GenericDataLocation
        {
            sh_get_known_folder_path!(
                FOLDERID_ProgramData,
                mut path,
                {
                    if location != GenericConfigLocation && location != GenericDataLocation {
                        self.append_organization_and_app(&mut path);
                    }
                    dirs.push(path);
                },
                {}
            );
            let path = env::current_exe()?;
            match path.parent() {
                Some(parent) => {
                    let mut parent: PathBuf = parent.into();
                    dirs.push(parent.clone());
                    parent.push("data");
                    dirs.push(parent);
                }
                _ => (),
            }
        }
        Ok(dirs)
    }
}

/// Detect if `path` is an executable (based on
/// [GetBinaryType](https://msdn.microsoft.com/ru-ru/library/windows/desktop/aa364819.aspx)).
fn is_executable<P>(path: P) -> bool
where
    P: AsRef<OsStr>,
{
    unsafe {
        let name = path
            .as_ref()
            .encode_wide()
            .chain(Some(0))
            .collect::<Vec<_>>();
        let mut bt: DWORD = 0;
        GetBinaryTypeW(name.as_ptr(), &mut bt) != 0
    }
}

#[inline]
#[doc(hidden)]
pub fn find_executable_in_paths_impl<S, P>(name: S, paths: P) -> Option<Vec<PathBuf>>
where
    S: Into<String>,
    P: AsRef<Vec<PathBuf>>,
{
    let name = name.into();
    let path = PathBuf::from(&name);

    // On Windows, if the name does not have a suffix or a suffix not in PATHEXT ("xx.foo"),
    // append suffixes from PATHEXT. If %PATHEXT% does not contain .exe, it is either empty or distorted.
    let mut exe_extensions = match env::var("PATHEXT") {
        Ok(pathext) => env::split_paths(&pathext)
            .map(|e| {
                e.to_str()
                    .unwrap()
                    .to_lowercase()
                    .get(1..)
                    .unwrap()
                    .to_string()
            })
            .collect(),
        _ => Vec::new(),
    };
    let exe = "exe".into();
    if exe_extensions.is_empty() || !exe_extensions.contains(&exe) {
        exe_extensions = vec![exe, "com".into(), "bat".into(), "cmd".into()];
    }

    // Check absolute paths
    if path.is_absolute() {
        if is_executable(&name) {
            return Some(vec![path]);
        } else {
            let mut res = Vec::new();
            for ext in &exe_extensions {
                let mut full_path = path.clone();
                full_path.set_extension(ext);
                if is_executable(&full_path) {
                    res.push(full_path);
                }
            }
            return if res.is_empty() { None } else { Some(res) };
        }
    }

    // Check paths
    let mut paths = paths.as_ref().clone();
    paths.retain(|p| !p.to_str().unwrap().is_empty() && p.is_dir());
    let paths = {
        let mut paths2 = Vec::new();
        for path in paths {
            // Remove '\\?\' prefix
            let canonicalized = path.canonicalize().unwrap();
            let path = PathBuf::from(canonicalized.to_str().unwrap().get(4..).unwrap());
            if !paths2.contains(&path) {
                paths2.push(path);
            }
        }
        paths2
    };

    // At first search the provided name
    let mut res = Vec::new();
    for mut path in paths.to_owned() {
        path.push(&name);
        if is_executable(&path) {
            res.push(path);
        }
    }

    // Then check if an extension could be appended
    if path.extension().is_none() {
        for mut path in paths.to_owned() {
            path.push(&name);
            for ext in &exe_extensions {
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

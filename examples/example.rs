extern crate standard_paths;

use standard_paths::*;
use standard_paths::LocationType::*;

fn main() {
    let locations = vec![
        ("Desktop",         DesktopLocation),
        ("Documents",       DocumentsLocation),
        ("Fonts",           FontsLocation),
        ("Applications",    ApplicationsLocation),
        ("Music",           MusicLocation),
        ("Movies",          MoviesLocation),
        ("Pictures",        PicturesLocation),
        ("Temp",            TempLocation),
        ("Home",            HomeLocation),
        ("Data",            DataLocation),
        ("Cache",           CacheLocation),
        ("Generic Data",    GenericDataLocation),
        ("Runtime",         RuntimeLocation),
        ("Config",          ConfigLocation),
        ("Download",        DownloadLocation),
        ("Generic Cache",   GenericCacheLocation),
        ("Generic Config",  GenericConfigLocation),
        ("App Data",        AppDataLocation),
        ("App Config",      AppConfigLocation),
        ("App Local Data",  AppLocalDataLocation)
    ];

    let sl = StandardLocation::new_with_names("app", "org");
    
    println!("Listing writable locations:");
    for (name, value) in locations {
        println!("{:>14}: {:?}", name, sl.writable_location(value.clone()).unwrap_or("".into()));
    }
}

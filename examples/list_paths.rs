extern crate standard_paths;

use standard_paths::*;
use standard_paths::LocationType::*;

fn main() {
    let locations = vec![
        ("Home",            HomeLocation),
        ("Desktop",         DesktopLocation),
        ("Documents",       DocumentsLocation),
        ("Download",        DownloadLocation),
        ("Movies",          MoviesLocation),
        ("Music",           MusicLocation),
        ("Pictures",        PicturesLocation),
        ("Applications",    ApplicationsLocation),
        ("Fonts",           FontsLocation),
        ("Runtime",         RuntimeLocation),
        ("Temp",            TempLocation),
        ("Generic Data",    GenericDataLocation),
        ("App Data",        AppDataLocation),
        ("App Local Data",  AppLocalDataLocation),
        ("Generic Cache",   GenericCacheLocation),
        ("App Cache",       AppCacheLocation),
        ("Config",          ConfigLocation),
        ("Generic Config",  GenericConfigLocation),
        ("App Config",      AppConfigLocation)
    ];

    let sl = StandardPaths::new_with_names("app", "org");
    
    println!("\nListing standard locations:");
    for &(ref name, ref value) in &locations {
        println!("{:>14}: {:?}", name, sl.standard_locations(value.clone()).unwrap_or(Vec::new()));
    }

    println!("\nListing writable locations:");
    for &(ref name, ref value) in &locations {
        println!("{:>14}: {:?}", name, sl.writable_location(value.clone()).unwrap_or("".into()));
    }
}

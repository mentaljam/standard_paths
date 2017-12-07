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
        match sl.standard_locations(value.clone()) {
            Some(paths) => println!("{:>14}: \"{}\"", name, paths.iter()
                                                                .map(|p| p.to_str().unwrap())
                                                                .collect::<Vec<_>>()
                                                                .join("\", \"")
                                                                ),
            _ => println!("{:>14}: None", name)
        }
    }

    println!("\nListing writable locations:");
    for &(ref name, ref value) in &locations {
        match sl.writable_location(value.clone()) {
            Some(path) => println!("{:>14}: \"{}\"", name, path.to_str().unwrap()),
            _ => println!("{:>14}: None", name)
        }
    }
}

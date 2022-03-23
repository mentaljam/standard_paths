extern crate standard_paths;

use standard_paths::LocationType::*;
use standard_paths::*;

fn main() {
    let locations = vec![
        ("Home", HomeLocation),
        ("Desktop", DesktopLocation),
        ("Documents", DocumentsLocation),
        ("Download", DownloadLocation),
        ("Movies", MoviesLocation),
        ("Music", MusicLocation),
        ("Pictures", PicturesLocation),
        ("Applications", ApplicationsLocation),
        ("Fonts", FontsLocation),
        ("Runtime", RuntimeLocation),
        ("Temp", TempLocation),
        ("Generic Data", GenericDataLocation),
        ("App Data", AppDataLocation),
        ("App Local Data", AppLocalDataLocation),
        ("Generic Cache", GenericCacheLocation),
        ("App Cache", AppCacheLocation),
        ("Config", ConfigLocation),
        ("Generic Config", GenericConfigLocation),
        ("App Config", AppConfigLocation),
    ];

    let sl = StandardPaths::new("app", "org");

    println!("\nListing standard locations:");
    for (name, value) in &locations {
        match sl.standard_locations(*value) {
            Ok(paths) => println!(
                "{:>14}: \"{}\"",
                name,
                paths
                    .iter()
                    .map(|p| p.to_str().unwrap())
                    .collect::<Vec<_>>()
                    .join("\", \"")
            ),
            Err(err) => println!("{:>14}: {}", name, err),
        }
    }

    println!("\nListing writable locations:");
    for (name, value) in &locations {
        match sl.writable_location(*value) {
            Ok(path) => println!("{:>14}: \"{}\"", name, path.to_str().unwrap()),
            Err(err) => println!("{:>14}: {}", name, err),
        }
    }
}

use standard_paths::*;

fn main() {
    let locations = vec![
        ("Home", LocationType::HomeLocation),
        ("Desktop", LocationType::DesktopLocation),
        ("Documents", LocationType::DocumentsLocation),
        ("Download", LocationType::DownloadLocation),
        ("Movies", LocationType::MoviesLocation),
        ("Music", LocationType::MusicLocation),
        ("Pictures", LocationType::PicturesLocation),
        ("Applications", LocationType::ApplicationsLocation),
        ("Fonts", LocationType::FontsLocation),
        ("Runtime", LocationType::RuntimeLocation),
        ("Temp", LocationType::TempLocation),
        ("Generic Data", LocationType::GenericDataLocation),
        ("App Data", LocationType::AppDataLocation),
        ("App Local Data", LocationType::AppLocalDataLocation),
        ("Generic Cache", LocationType::GenericCacheLocation),
        ("App Cache", LocationType::AppCacheLocation),
        ("Config", LocationType::ConfigLocation),
        ("Generic Config", LocationType::GenericConfigLocation),
        ("App Config", LocationType::AppConfigLocation),
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

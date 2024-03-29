use argparse::{ArgumentParser, Store, StoreTrue};
use std::process;

use standard_paths::*;

fn main() {
    let mut app_name = String::new();
    let mut org_name = String::new();
    let mut file = String::new();
    let mut location = String::new();
    let mut option = String::from("LocateFile");
    let mut locate_all = false;
    {
        let mut ap = ArgumentParser::new();
        ap.set_description("Example of usage of the standard_paths locate methods.");
        ap.refer(&mut app_name)
            .add_argument("app_name", Store, "name of the application")
            .required();
        ap.refer(&mut org_name)
            .add_argument("org_name", Store, "name of the organization")
            .required();
        ap.refer(&mut file)
            .add_argument("file", Store, "file name to search")
            .required();
        ap.refer(&mut location)
            .add_argument("location", Store, "type of location")
            .required();
        ap.refer(&mut option).add_option(
            &["-o", "--option"],
            Store,
            "locate option, default is 'LocateFile'",
        );
        ap.refer(&mut locate_all).add_option(
            &["-a", "--all"],
            StoreTrue,
            "locate all, default is false",
        );
        ap.parse_args_or_exit();
    }

    let location = match location.as_str() {
        "HomeLocation" => LocationType::HomeLocation,
        "DesktopLocation" => LocationType::DesktopLocation,
        "DocumentsLocation" => LocationType::DocumentsLocation,
        "DownloadLocation" => LocationType::DownloadLocation,
        "MoviesLocation" => LocationType::MoviesLocation,
        "MusicLocation" => LocationType::MusicLocation,
        "PicturesLocation" => LocationType::PicturesLocation,
        "ApplicationsLocation" => LocationType::ApplicationsLocation,
        "FontsLocation" => LocationType::FontsLocation,
        "RuntimeLocation" => LocationType::RuntimeLocation,
        "TempLocation" => LocationType::TempLocation,
        "GenericDataLocation" => LocationType::GenericDataLocation,
        "AppDataLocation" => LocationType::AppDataLocation,
        "AppLocalDataLocation" => LocationType::AppLocalDataLocation,
        "GenericCacheLocation" => LocationType::GenericCacheLocation,
        "AppCacheLocation" => LocationType::AppCacheLocation,
        "ConfigLocation" => LocationType::ConfigLocation,
        "GenericConfigLocation" => LocationType::GenericConfigLocation,
        "AppConfigLocation" => LocationType::AppConfigLocation,
        _ => {
            eprintln!("Bad location type '{location}', see the documentation for valid values");
            process::exit(1)
        }
    };

    let option = match option.as_str() {
        "LocateBoth" => LocateOption::LocateBoth,
        "LocateFile" => LocateOption::LocateFile,
        "LocateDirectory" => LocateOption::LocateDirectory,
        _ => {
            eprintln!("Bad locate option '{option}', see the documentation for valid values");
            process::exit(1)
        }
    };

    let sp = StandardPaths::new(app_name, org_name);
    if locate_all {
        match sp.locate_all(location, &file, option) {
            Ok(paths) => {
                if let Some(paths) = paths {
                    let paths = paths
                        .iter()
                        .map(|p| p.to_str().unwrap())
                        .collect::<Vec<_>>()
                        .join(r#"", ""#);
                    println!(r#""{paths}""#);
                    process::exit(0)
                }
            }
            Err(err) => {
                eprintln!("{err}");
                process::exit(2)
            }
        }
    } else {
        match sp.locate(location, &file, option) {
            Ok(path) => {
                if path.is_some() {
                    println!(r#""{}""#, path.unwrap().to_str().unwrap());
                    process::exit(0)
                }
            }
            Err(err) => {
                eprintln!("{err}");
                process::exit(2)
            }
        }
    }

    println!("No entries found for '{file}'");
}

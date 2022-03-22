extern crate argparse;
extern crate standard_paths;

use argparse::{ArgumentParser, Store, StoreTrue};
use standard_paths::LocateOption::*;
use standard_paths::LocationType::*;
use standard_paths::*;
use std::process;

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
        "HomeLocation" => HomeLocation,
        "DesktopLocation" => DesktopLocation,
        "DocumentsLocation" => DocumentsLocation,
        "DownloadLocation" => DownloadLocation,
        "MoviesLocation" => MoviesLocation,
        "MusicLocation" => MusicLocation,
        "PicturesLocation" => PicturesLocation,
        "ApplicationsLocation" => ApplicationsLocation,
        "FontsLocation" => FontsLocation,
        "RuntimeLocation" => RuntimeLocation,
        "TempLocation" => TempLocation,
        "GenericDataLocation" => GenericDataLocation,
        "AppDataLocation" => AppDataLocation,
        "AppLocalDataLocation" => AppLocalDataLocation,
        "GenericCacheLocation" => GenericCacheLocation,
        "AppCacheLocation" => AppCacheLocation,
        "ConfigLocation" => ConfigLocation,
        "GenericConfigLocation" => GenericConfigLocation,
        "AppConfigLocation" => AppConfigLocation,
        _ => {
            eprintln!(
                "Bad location type '{}', see the documentation for valid values",
                location
            );
            process::exit(1)
        }
    };

    let option = match option.as_str() {
        "LocateBoth" => LocateBoth,
        "LocateFile" => LocateFile,
        "LocateDirectory" => LocateDirectory,
        _ => {
            eprintln!(
                "Bad locate option '{}', see the documentation for valid values",
                option
            );
            process::exit(1)
        }
    };

    let sp = StandardPaths::new_with_names(app_name, org_name);
    if locate_all {
        match sp.locate_all(location, &file, option) {
            Ok(paths) => {
                if paths.is_some() {
                    println!(
                        "\"{}\"",
                        paths
                            .unwrap()
                            .iter()
                            .map(|p| p.to_str().unwrap())
                            .collect::<Vec<_>>()
                            .join("\", \"")
                    );
                    process::exit(0)
                }
            }
            Err(err) => {
                eprintln!("{}", err);
                process::exit(2)
            }
        }
    } else {
        match sp.locate(location, &file, option) {
            Ok(path) => {
                if path.is_some() {
                    println!("\"{}\"", path.unwrap().to_str().unwrap());
                    process::exit(0)
                }
            }
            Err(err) => {
                eprintln!("{}", err);
                process::exit(2)
            }
        }
    }

    println!("No entries found for '{}'", file);
}

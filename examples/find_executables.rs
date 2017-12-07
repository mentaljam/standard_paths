extern crate standard_paths;

use std::env;
use std::process;
use standard_paths::*;

fn main() {
    let args = env::args().skip(1).collect::<Vec<_>>();
    if args.len() == 0 {
        println!("\n    Usage: find_executable <path1> [path2 [path3 ...]]\n");
        process::exit(0);
    }

    let mut ind: usize = 0;
    for exe in &args {
        let len = exe.len();
        if len < 16 && ind < len {
            ind = len;
        }
    }

    println!("Searching executables:");
    for exe in args {
        let fexe = format!("{:>1$}", exe, ind);
        match StandardPaths::find_executable(exe.clone()) {
            Some(paths) => println!("{}: \"{}\"", fexe, paths.iter()
                                                        .map(|p| p.to_str().unwrap())
                                                        .collect::<Vec<_>>()
                                                        .join("\", \"")),
            _ => println!("{}: not found", fexe)
        }
    }
}

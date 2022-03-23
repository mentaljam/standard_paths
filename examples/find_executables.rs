use std::{env, process};

use standard_paths::*;

fn main() {
    let args = env::args().skip(1).collect::<Vec<_>>();
    if args.is_empty() {
        println!("\n    Usage: find_executable <path1> [path2 [path3 ...]]\n");
        process::exit(0);
    }

    let mut ind: usize = 0;
    let args = {
        let mut args2 = Vec::new();
        for exe in &args {
            if !args2.contains(exe) {
                args2.push(exe.clone());
                let len = exe.len();
                if len < 16 && ind < len {
                    ind = len;
                }
            }
        }
        args2
    };

    println!("Searching executables:");
    for exe in args {
        let fexe = format!("{:>1$}", exe, ind);
        match StandardPaths::find_executable(exe.clone()) {
            Some(paths) => println!(
                "{}: \"{}\"",
                fexe,
                paths
                    .iter()
                    .map(|p| p.to_str().unwrap())
                    .collect::<Vec<_>>()
                    .join("\", \"")
            ),
            _ => println!("{}: not found", fexe),
        }
    }
}

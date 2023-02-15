use crate::{
    config::constants,
    util::{delete_all::delete_all, delete_folder::delete_folder, shred_file::shred_file},
};
use std::{fs, path::Path};

mod config;
mod util;

fn main() {
    // get passed arguments
    let args: Vec<String> = std::env::args().collect();

    // drop the first argument (the path to the executable)
    let args = &args[1..];

    // check if there are any arguments
    if args.is_empty() {
        println!("srm: missing operand\nTry 'srm --help' for more information.");
        std::process::exit(0);
    }

    let mut paths: Vec<String> = Vec::new();
    let mut recursive = false;
    let mut passes = 40;
    let mut verbose = false;

    if constants::DEBUG {
        println!("--DEBUG--")
    }

    // loop through arguments
    for (counter, arg) in args.iter().enumerate() {
        if constants::DEBUG {
            println!("{}: {}", counter, arg);
        }

        // check if the argument is a path
        let (exists, is_dir) = (Path::new(arg).exists(), Path::new(arg).is_dir());
        if arg == "*" || arg == "./*" || arg == ".\\*" {
            paths.push(arg.to_string());
            continue;
        } else if exists || is_dir {
            paths.push(arg.to_string());
        } else {
            match arg.as_str() {
                "-r" | "--recursive" => recursive = true,
                "-v" | "--verbose" => verbose = true,
                "-p" | "--passes" => passes = args[counter + 1].parse::<u32>().unwrap(),
                "--version" => {
                    constants::version();
                    std::process::exit(0);
                }
                "--help" => {
                    constants::help();
                    std::process::exit(0);
                }
                _ => {
                    if match args[counter - 1].as_str() {
                        "-p" | "--passes" => continue,
                        _ => false,
                    } {
                    } else if arg.contains('-') && !arg.contains('/') && !arg.contains('\\') {
                        // if the argument is not a valid argument
                        println!(
                            "srm: invalid option '{}'\nTry 'srm --help' for more information.",
                            arg
                        );
                        std::process::exit(0);
                    } else {
                        // if the argument is not a path
                        println!("srm: cannot remove '{}': No such file or directory", arg);
                        std::process::exit(0);
                    }
                }
            }
        }
    }

    if constants::DEBUG {
        println!("\nPath: {:?}", paths);
        println!("Recursive: {}", recursive);
        println!("Passes: {}", passes);
        println!("Verbose: {}\n--DEBUG--", verbose);
    }

    if paths.is_empty() {
        println!("srm: missing operand\nTry 'srm --help' for more information.");
        std::process::exit(0);
    }
    for path in paths {
        if ["/", "c", "c:", "c:\\", "c:/"].contains(&path.to_lowercase().as_str()) {
            println!("'{}': Cannot delete root directory.", path);
            continue;
        } else if path == "*" || path == "./*" || path == ".\\*" {
            delete_all(passes, verbose, recursive);
            break;
        }

        let (is_file, is_dir) = (Path::new(&path).is_file(), Path::new(&path).is_dir());
        if is_file {
            shred_file(&path, passes, verbose);
        } else if is_dir {
            if recursive {
                delete_folder(&path, passes, verbose);
                if verbose {
                    print!("Deleting directory '{}' . . .\t\t", path);
                }
                fs::remove_dir_all(&path).unwrap();
                if verbose {
                    println!("Done");
                }
            } else {
                println!("'{}': Is a directory", path);
            }
        }
    }

    if constants::DEBUG {
        println!("- DEBUG - Done.");
    }
}

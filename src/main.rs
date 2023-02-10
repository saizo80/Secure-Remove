use rand::{RngCore, SeedableRng};
use rand_chacha::ChaChaRng;
use std::{
    fs,
    io::{Seek, SeekFrom, Write},
    path,
};

const DEBUG: bool = false;
const VERSION: &str = "2.0.0";
const AUTHOR: &str = "Olivier Thornton";
const LICENSE: &str = "GPLV3+: GNU GPL version 3 or later <http://gnu.org/licenses/gpl.html>.";

fn main() {
    // get passed arguments
    let args: Vec<String> = std::env::args().collect();

    // drop the first argument (the path to the executable)
    let args = &args[1..];

    // check if there are any arguments
    if args.len() == 0 {
        println!("srm: missing operand\nTry 'srm --help' for more information.");
        std::process::exit(0);
    }

    let mut path = String::new();
    let mut recursive = false;
    let mut passes = 10;
    let mut verbose = false;

    let mut counter = 0;

    if DEBUG {
        println!("--DEBUG--")
    }

    // loop through arguments
    for arg in args {
        if DEBUG {
            println!("{}: {}", counter, arg);
        }

        if counter == args.len() - 1
            && arg != "-r"
            && arg != "-v"
            && arg != "-p"
            && arg != "--help"
            && arg != "--version"
            && arg != "--verbose"
            && arg != "--recursive"
            && arg != "--passes"
        {
            path = arg.to_string();
        }

        if arg == "-r" || arg == "--recursive" {
            recursive = true;
        }

        if arg == "-v" || arg == "--verbose" {
            verbose = true;
        }

        if arg == "-p" || arg == "--passes" {
            passes = args[counter + 1].parse::<u16>().unwrap();
            counter += 1;
        }

        if arg == "--version" {
            version();
            std::process::exit(0);
        }

        if arg == "--help" {
            help();
            std::process::exit(0);
        }

        if arg.contains("-")
            && arg != "-r"
            && arg != "-v"
            && arg != "-p"
            && arg != "--help"
            && arg != "--version"
            && arg != "--verbose"
            && arg != "--recursive"
            && arg != "--passes"
            && !arg.contains("/")
            && !arg.contains("\\")
        {
            if DEBUG {
                println!(
                    "'{}': \nFile exists: {}\nDirectory Exists: {}",
                    arg,
                    path::Path::new(arg).exists(),
                    path::Path::new(arg).is_dir()
                );
            }

            if !path::Path::new(arg).exists() && !path::Path::new(arg).is_dir() {
                println!(
                    "srm: invalid option '{}'\nTry 'srm --help' for more information.",
                    arg
                );
                std::process::exit(0);
            }
        }

        counter += 1;
    }

    if DEBUG {
        println!("\nPath: {}", path);
        println!("Recursive: {}", recursive);
        println!("Passes: {}", passes);
        println!("Verbose: {}\n--DEBUG--", verbose);
    }

    if path == "" {
        println!("srm: missing operand\nTry 'srm --help' for more information.");
        std::process::exit(0);
    } else if path == "/" || path.to_lowercase() == "c" || path.to_lowercase() == "c:" {
        println!("Cannot delete root directory.");
    } else if path == "*" || path == "./*" || path == ".\\*" {
        // get all files and folders in the working directory
        let mut entries = fs::read_dir(".").unwrap();

        // loop through the files
        while let Some(entry) = entries.next() {
            let object = entry.unwrap();
            if DEBUG {
                println!("- DEBUG - File: {}", &object.file_name().to_string_lossy());
            }
            // if object is a directory
            if object.file_type().unwrap().is_dir() {
                // if recursive is true
                if recursive {
                    // delete the folder
                    delete_folder(
                        &object.file_name().to_string_lossy().to_string(),
                        passes,
                        verbose,
                    );
                } else if !recursive {
                    // if recursive is false, skip the folder
                    println!(
                        "'{}' is a directory . . . Skipping",
                        object.file_name().to_string_lossy()
                    );
                }
            } else if object.file_type().unwrap().is_file() {
                // if object is a file, delete it
                delete_file(
                    &object.file_name().to_string_lossy().to_string(),
                    passes,
                    verbose,
                );
            }
        }
    } else if path::Path::new(&path).is_dir() {
        if recursive {
            if DEBUG {
                println!("- DEBUG - Path is a directory, calling delete_folder");
            }
            // if path is a directory and recursive is true, delete the folder
            delete_folder(&path, passes, verbose);
            if verbose {
                print!("Deleting directory '{}' . . .\t\t", path);
            }
            // delete the directory
            fs::remove_dir_all(path).unwrap();
            if verbose {
                println!("Done");
            }
        } else if !recursive {
            // if path is a directory and recursive is false, skip the folder
            println!("srm: cannot remove '{}': Is a directory", path);
        }
    } else {
        if path::Path::new(&path).is_file() {
            delete_file(&path, passes, verbose)
        } else {
            println!("srm: cannot remove '{}': No such file or directory", path);
        }
    }

    if DEBUG {
        println!("- DEBUG - Done.");
    }
}

fn delete_folder(path: &String, passes: u16, verbose: bool) {
    // if path ends with . or .., skip it
    if path.ends_with(".") || path.ends_with("..") {
        return;
    }

    if DEBUG {
        println!("- DEBUG - Deleting folder: '{}'", path);
    }

    // get all files and folders in the directory
    let mut entries = fs::read_dir(path).unwrap();

    // loop through the files
    while let Some(entry) = entries.next() {
        let object = entry.unwrap();

        if DEBUG {
            println!("- DEBUG - File: {}", &object.file_name().to_string_lossy());
        }

        if object.file_type().unwrap().is_dir() {
            // create the full path to the directory
            let new_path = path.to_string() + "/" + &object.file_name().to_string_lossy();

            delete_folder(&new_path, passes, verbose);

            if verbose {
                print!("Deleting directory '{}' . . .\t\t", path);
            }

            // delete the directory
            fs::remove_dir_all(new_path).unwrap();

            if verbose {
                println!("Done");
            }
        } else if object.file_type().unwrap().is_file() {
            // create the full path to the file
            let new_path = path.to_string() + "/" + &object.file_name().to_string_lossy();

            delete_file(&new_path, passes, verbose);
        }
    }
}

fn delete_file(path: &String, passes: u16, verbose: bool) {
    if DEBUG {
        println!("- DEBUG - Deleting file: '{}'", path);
    }

    // open the file for reading and writing
    let mut file = fs::OpenOptions::new()
        .read(true)
        .write(true)
        .open(path)
        .unwrap();

    if verbose {
        print!("Deleting file '{}' . . .\t\t", path);
    }

    // get the file size
    let file_size = file.metadata().unwrap().len();

    // create a buffer to hold the random data
    let mut buffer = vec![0; file_size as usize];

    // create a random generator
    let mut rng = ChaChaRng::from_entropy();

    // loop through the file passes times
    for _ in 0..passes {
        // fill the buffer with random secure data using chacha
        rng.fill_bytes(&mut buffer);
        // write the buffer to the file
        file.write_all(&buffer).unwrap();
        // flush the file
        file.flush().unwrap();
        // seek to the beginning of the file
        file.seek(SeekFrom::Start(0)).unwrap();
    }

    // delete the file
    fs::remove_file(path).unwrap();

    if verbose {
        println!("Done");
    }
}

fn version() {
    println!("srm {}", VERSION);
    println!("License {}", LICENSE);
    println!("This is free software: you are free to change and redistribute it.\nThere is NO WARRANTY, to the extent permitted by law.");
    println!("\nWritten by {}.", AUTHOR);
}

fn help() {
    println!("Usage: srm [OPTION]... FILE...");
    println!("Securely remove files or directories.\n");
    println!("  -r, --recursive\tremove directories and their contents recursively");
    println!("  -p, --passes\t\tset the number of passes (default: 10)");
    println!("  -v, --verbose\t\texplain what is being done");
    println!("      --help\t\tdisplay this help and exit");
    println!("      --version\t\toutput version information and exit");
}

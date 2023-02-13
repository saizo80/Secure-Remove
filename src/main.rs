use std::{fs, path::Path};

mod constants;

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

    let mut path = String::new();
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

        match arg.as_str() {
            "-r" | "--recursive" => recursive = true,
            "-v" | "--verbose" => verbose = true,
            "-p" | "--passes" => passes = args[counter + 1].parse::<u32>().unwrap(),
            "--version" => {
                version();
                std::process::exit(0);
            }
            "--help" => {
                help();
                std::process::exit(0);
            }
            _ => {}
        }

        if counter == args.len() - 1 && !constants::CHECK_ARGS.contains(&arg.as_str()) {
            path = arg.to_string();
        }

        if arg.contains("-")
            && !constants::CHECK_ARGS.contains(&arg.as_str())
            && !arg.contains("/")
            && !arg.contains("\\")
        {
            let (exists, is_dir) = (Path::new(arg).exists(), Path::new(arg).is_dir());

            if constants::DEBUG {
                println!(
                    "'{}': \nFile exists: {}\nDirectory Exists: {}",
                    arg, exists, is_dir
                );
            }

            if !exists && !is_dir {
                println!(
                    "srm: invalid option '{}'\nTry 'srm --help' for more information.",
                    arg
                );
                std::process::exit(0);
            }
        }
    }

    if constants::DEBUG {
        println!("\nPath: {}", path);
        println!("Recursive: {}", recursive);
        println!("Passes: {}", passes);
        println!("Verbose: {}\n--DEBUG--", verbose);
    }

    if path.is_empty() {
        println!("srm: missing operand\nTry 'srm --help' for more information.");
        std::process::exit(0);
    } else if path == "/" || path.to_lowercase() == "c" || path.to_lowercase() == "c:" {
        println!("Cannot delete root directory.");
    } else if path == "*" || path == "./*" || path == ".\\*" {
        // get all files and folders in the working directory
        let entries = fs::read_dir(".").unwrap();

        // loop through the files
        for entry in entries {
            let object = entry.unwrap();
            if constants::DEBUG {
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
                shred_file(
                    &object.file_name().to_string_lossy().to_string(),
                    passes,
                    verbose,
                );
            }
        }
    } else if Path::new(&path).is_dir() {
        if recursive {
            if constants::DEBUG {
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
    } else if Path::new(&path).is_file() {
        shred_file(&path, passes, verbose)
    } else {
        println!("srm: cannot remove '{}': No such file or directory", path);
    }

    if constants::DEBUG {
        println!("- DEBUG - Done.");
    }
}

fn delete_folder(path: &String, passes: u32, verbose: bool) {
    // if path ends with . or .., skip it
    if path.ends_with('.') || path.ends_with("..") {
        return;
    }

    if constants::DEBUG {
        println!("- DEBUG - Deleting folder: '{}'", path);
    }

    // get all files and folders in the directory
    let entries = fs::read_dir(path).unwrap();

    // loop through the files
    for entry in entries {
        let object = entry.unwrap();

        if constants::DEBUG {
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

            shred_file(&new_path, passes, verbose);
        }
    }
}

fn shred_file(path: &String, passes: u32, verbose: bool) {
    if constants::DEBUG {
        println!("- DEBUG - Deleting file: '{}'", path);
    }

    let files = vec![path.to_string()];

    if verbose {
        print!("Deleting file '{}' . . .\t\t", path);
    }

    let config = file_shred::ShredConfig {
        files,
        confirmation_prompt: false,
        verbosity: file_shred::Verbosity::Quiet,
        keep_files: false,
        overwrite_count: passes,
        rename_count: passes,
        progress_bar: false,
    };
    file_shred::shred(&config).unwrap();

    if verbose {
        println!("Done");
    }
}

fn version() {
    println!("srm {}", constants::VERSION);
    println!("License {}", constants::LICENSE);
    println!("This is free software: you are free to change and redistribute it.\nThere is NO WARRANTY, to the extent permitted by law.");
    println!("\nWritten by {}.", constants::AUTHOR);
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

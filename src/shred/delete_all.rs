use crate::shred::{delete_folder::delete_folder, shred_file::shred_file};
use std::{fs, path::Path};

pub fn delete_all(passes: u32, verbose: bool, recursive: bool) {
    // get all files and folders in the working directory as a vector of strings
    let entries_raw = match fs::read_dir(".") {
        Ok(entries) => entries,
        Err(_) => {
            println!(
                "'{}': failed to read directory",
                std::env::current_dir().unwrap().to_string_lossy()
            );
            std::process::exit(0);
        }
    };
    let mut entries: Vec<String> = Vec::new();
    for entry in entries_raw {
        let temp = entry.unwrap().file_name().to_string_lossy().to_string();
        // if the entry does not begin with a dot, add it to the vector
        if !temp.starts_with('.') {
            entries.push(temp);
        }
    }

    for entry in entries {
        let path = Path::new(&entry);
        if path.is_file() {
            shred_file(&entry, passes, verbose);
        } else if path.is_dir() && recursive {
            match delete_folder(&entry, passes, verbose) {
                Ok(_) => {
                    if verbose {
                        print!("Deleting directory '{}' . . .\t\t", entry);
                    }
                    // delete the directory
                    match fs::remove_dir_all(&entry) {
                        Ok(_) => {
                            if verbose {
                                println!("Done");
                            }
                        }
                        Err(e) => {
                            println!("'{}': {}", entry, e);
                            continue;
                        }
                    };
                }
                Err(_) => {
                    println!("'{}': Failed to delete directory", entry);
                    continue;
                }
            };
        } else if path.is_dir() && !recursive {
            println!("srm: cannot remove '{}': Is a directory", entry);
        }
    }
}

use crate::{config::constants, shred::shred_file::shred_file};
use std::fs;

pub fn delete_folder(path: &String, passes: u32, verbose: bool) -> Result<(), std::io::Error> {
    if constants::DEBUG {
        println!("- DEBUG - Deleting folder: '{}'", path);
    }

    // get all files and folders in the directory
    let entries = match fs::read_dir(path) {
        Ok(entries) => entries,
        Err(_) => {
            return Err(std::io::Error::new(
                std::io::ErrorKind::Other,
                "Failed to read directory",
            ));
        }
    };

    // loop through the files
    for entry in entries {
        let object = entry?;

        if constants::DEBUG {
            println!("- DEBUG - File: {}", &object.file_name().to_string_lossy());
        }

        if object.file_type()?.is_dir() {
            // create the full path to the directory
            let new_path = path.to_string() + "/" + &object.file_name().to_string_lossy();

            match delete_folder(&new_path, passes, verbose) {
                Ok(_) => {
                    if verbose {
                        print!("Deleting directory '{}' . . .\t\t", new_path);
                    }

                    // delete the directory
                    match fs::remove_dir_all(&new_path) {
                        Ok(_) => {
                            if verbose {
                                println!("Done");
                            }
                        }
                        Err(e) => {
                            println!("'{}': {}", new_path, e);
                            continue;
                        }
                    };
                }
                Err(_) => {
                    println!("'{}': Failed to delete directory", new_path);
                    continue;
                }
            }
        } else if object.file_type().unwrap().is_file() {
            // create the full path to the file
            let new_path = path.to_string() + "/" + &object.file_name().to_string_lossy();

            shred_file(&new_path, passes, verbose);
        }
    }
    Ok(())
}

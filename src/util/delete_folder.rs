use crate::{config::constants, util::shred_file::shred_file};
use std::fs;

pub fn delete_folder(path: &String, passes: u32, verbose: bool) {
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

use crate::util::{delete_folder::delete_folder, shred_file::shred_file};
use std::{fs, path::Path};

pub fn delete_all(passes: u32, verbose: bool, recursive: bool) {
    // get all files and folders in the working directory as a vector of strings
    let entries_raw = fs::read_dir(".").unwrap();
    let mut entries: Vec<String> = Vec::new();
    for entry in entries_raw {
        entries.push(entry.unwrap().file_name().to_string_lossy().to_string());
    }

    for entry in entries {
        let path = Path::new(&entry);
        if path.is_file() {
            shred_file(&entry, passes, verbose);
        } else if path.is_dir() && recursive {
            delete_folder(&entry, passes, verbose);

            if verbose {
                print!("Deleting directory '{}' . . .\t\t", entry);
            }
            // delete the directory
            fs::remove_dir_all(entry).unwrap();
            if verbose {
                println!("Done");
            }
        } else if path.is_dir() && !recursive {
            println!("srm: cannot remove '{}': Is a directory", entry);
        }
    }
}

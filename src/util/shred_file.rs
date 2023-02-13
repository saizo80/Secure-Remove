use crate::config::constants;

pub fn shred_file(path: &String, passes: u32, verbose: bool) {
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

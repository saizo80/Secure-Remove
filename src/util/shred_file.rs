use std::{
    fs,
    io::{Seek, Write},
};

use crate::config::constants;
use rand::{RngCore, SeedableRng};
use rand_chacha::ChaChaRng;

pub fn shred_file(path: &String, passes: u32, verbose: bool) {
    if constants::DEBUG {
        println!("- DEBUG - Deleting file: '{}'", path);
    }

    if verbose {
        print!("Deleting file '{}' . . .\t\t", path);
    }

    // open the file for reading and overwriting
    let mut file = fs::OpenOptions::new()
        .read(true)
        .write(true)
        .append(false)
        .open(path)
        .unwrap();

    // create a buffer to hold the data
    let size = file.metadata().unwrap().len();
    let mut buffer = vec![0; size.min(512) as usize];

    // create a random number generator
    let mut rng = ChaChaRng::from_entropy();

    // loop through the passes
    #[allow(clippy::single_element_loop)]
    for _ in [0..passes] {
        // seek to the beginning of the file
        file.rewind().unwrap();

        // overwrite the file with random data, 512 bytes at a time until the file is empty
        let mut offset = 0;
        while offset < size {
            // fill the buffer with random data
            rng.fill_bytes(&mut buffer);

            // write the buffer to the file
            file.write_all(&buffer).unwrap();

            // increment the offset
            offset += buffer.len() as u64;
        }

        // flush the file
        file.flush().unwrap();
    }

    // delete the file
    fs::remove_file(path).unwrap();

    if verbose {
        println!("Done");
    }
}

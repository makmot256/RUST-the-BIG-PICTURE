// patching.rs

use std::fs::File;
use std::io::{Read, Write};
use std::path::Path;

// Patching a file
fn patch_file(file_path: &str, patch_path: &str) -> Result<(), String> {
    let mut file = match File::open(file_path) {
        Ok(file) => file,
        Err(err) => return Err(err.to_string()),
    };

    let mut patch = match File::open(patch_path) {
        Ok(patch) => patch,
        Err(err) => return Err(err.to_string()),
    };

    let mut file_data = String::new();
    match file.read_to_string(&mut file_data) {
        Ok(_) => (),
        Err(err) => return Err(err.to_string()),
    }

    let mut patch_data = String::new();
    match patch.read_to_string(&mut patch_data) {
        Ok(_) => (),
        Err(err) => return Err(err.to_string()),
    }

    let patched_data = patch::patch(file_data.as_bytes(), patch_data.as_bytes())?;

    let mut patched_file = match File::create(file_path) {
        Ok(file) => file,
        Err(err) => return Err(err.to_string()),
    };

    match patched_file.write_all(patched_data.as_bytes()) {
        Ok(_) => Ok(()),
        Err(err) => Err(err.to_string()),
    }
}

// Patches the file at the given path with the given patch
fn main() {
    let file_path = "example.txt";
    let patch_path = "example.patch";
    match patch_file(file_path, patch_path) {
        Ok(_) => println!("Patched file successfully"),
        Err(err) => println!("Error: {}", err),
    }
}

use std::env::args;
use std::io::{Read};
use std::fs::{self, File};

use image_viewer::bmp;

fn main()  {
    let mut args = args();
    args.next();
    let file_path = match args.next() {
        Some(file) => file,
        None       => {println!("Please enter an image file to open"); return }
    };

    if !fs::exists(&file_path).unwrap() {
        println!("Could not find file");
        return;
    }

    let i: usize = 
        (&file_path)
            .chars().rev().enumerate()
            .take_while(|(_, c)| c != &'.')
            .max_by(|a, b| a.0.cmp(&b.0))
            .unwrap().0;
    let file_type = &file_path[file_path.len()-1 - i..];
    
    match file_type {
        "bmp" => bmp::view_bmp(file_path),
        _     => {println!("Unknown file type"); return}
    }
}
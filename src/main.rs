use std::fs::{self, File, OpenOptions};
use std::fs::{DirEntry};
use std::io::BufReader;
use std::io::BufRead;
use std::io;
use std::path::Path;
use std::ffi::OsStr;
use std::io::Write;
use std::io::Seek;
use std::io::SeekFrom;


#[allow(dead_code)]
static GEN_TOC: bool = true;

fn main() {
    println!("Generate TOC: {}",GEN_TOC);
    static MAIN_PATH: &'static str = "/home/pedro/documents/markdown_notes";
    let path = Path::new(MAIN_PATH);
    transverse_directory(&path);
}

fn transverse_directory(path: &Path){
    let entries = fs::read_dir(path).unwrap();

    for entry in entries {
        let path = entry.unwrap().path();

        // Ignore hidden directories/files
        let os_str = OsStr::new(".git");
        let os_filename = path.file_name();
        let name = os_filename.unwrap().to_str().unwrap();
        if !name.starts_with(".") {
            if path.is_dir() {
                // TODO: Iterate over the other sub-directories
                transverse_directory(&path)
            } else {
                // TODO : Skip files named index.md | readme.md
                // TODO : Only handle '.md' files
                if name.ends_with(".md") {
                    // Print file name being handled
                    println!("{}", path.display());

                    // Handle this file
                    handle_file(&path);
                }
            }
        }
    }
}

fn handle_file(file_path: &Path){
    // Read trough the file
    let mut found_toc = false;
    let f = match File::open(file_path) {
        Ok(file) => {
            let br = BufReader::new(file);
            for line in br.lines() {
                if line.unwrap().contains("[toc]") {
                    found_toc = true;
                }
                //println!("{}", line.unwrap());
            }
        },
        Err(e) => {
            println!("Error: {:?}", e);
        }
    };

    // Add TOC in case it was missing
    if GEN_TOC {
        println!(" >Found TOC: {}", found_toc);
        add_toc(&file_path);
    }

    // Add extra line to improve readability of next file
    println!();
}

/**
* Adds a TOC entry to the first line of a given file
**/
fn add_toc(file_path: &Path){
    //let mut file = OpenOptions::new().write(true).open(&file_path).unwrap();
    //file.seek(SeekFrom::Start(0));
    //file.write_all(b"[TOC]\n\n");
}


use std::fs::{self, File, OpenOptions};
use std::io::BufReader;
use std::io::BufRead;
use std::path::Path;
use std::io::Write;
use std::io::Read;

// TODO: Change this constants into command line parameters
static GEN_TOC: bool = true;
static SKIP_INDEX: bool = true;
static SKIP_README: bool = true;
static MAIN_PATH: &'static str = "/home/pedro/documents/markdown_notes";

fn main() {
    println!("Generate TOC: {}",GEN_TOC);
    println!("Skip index.md files: {}",SKIP_INDEX);
    println!("Skip readme.md files: {}",SKIP_README);

    let path = Path::new(MAIN_PATH);
    transverse_directory(&path);
}

fn transverse_directory(path: &Path){
    let entries = fs::read_dir(path).unwrap();

    'nextfile: for entry in entries {
        let path = entry.unwrap().path();

        // Ignore hidden directories/files
        let os_filename = path.file_name();
        let name = os_filename.unwrap().to_str().unwrap();
        if !name.starts_with(".") {
            if path.is_dir() {
                // Iterate over the other sub-directories
                transverse_directory(&path)
            } else {
                // Only handle '.md' files
                if name.ends_with(".md") {

                    // Skip files named index.md | readme.md
                    match name.as_ref() {
                        "readme.md" => {
                            if SKIP_README{
                                continue;
                            }
                        },
                        "index.md" => {
                            if SKIP_INDEX{
                                continue;
                            }
                        },
                        _ => {
                            // All good, move along
                        }
                    }
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
    let _ = match File::open(file_path) {
        Ok(file) => {
            let br = BufReader::new(file);
            for line in br.lines() {
                if line.unwrap().to_lowercase().contains("[toc]") {
                    found_toc = true;
                }
            }
        },
        Err(e) => {
            println!("Error: {:?}", e);
        }
    };

    // Add TOC in case it was missing
    if GEN_TOC {
        println!(" >Found TOC: {}", found_toc);
        if !found_toc {
            add_toc(&file_path);
        }
    }

    // Add extra line to improve readability of next file
    println!();
}

/**
* Adds a TOC entry to the first line of a given file
**/
fn add_toc(file_path: &Path){
    // Load entire file into memory
    let mut file_read = OpenOptions::new().read(true)
        .open(&file_path)
        .expect("Unable to open file for reading");

    let mut contents = String::new();
    file_read.read_to_string(&mut contents).expect("Unable to read the file");


    // Write TOC Header and then the rest of the file back
    let mut file_write = OpenOptions::new().write(true).open(&file_path).unwrap();
    file_write.write_all(b"[TOC]\n\n").expect("Unable to write TOC");
    file_write.write_all(contents.as_bytes()).expect("Unable to re-write file");
}


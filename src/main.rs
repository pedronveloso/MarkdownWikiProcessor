use std::fs::{self, File, OpenOptions};
use std::fs::{DirEntry};
use std::io::BufReader;
use std::io::BufRead;
use std::io;
use std::path::Path;
use std::ffi::OsStr;


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
        let osname = path.file_name();
        let name = osname.unwrap().to_str();
        if !name.unwrap().starts_with(".") {
            println!("{}", path.display());
            if path.is_dir() {
                // TODO: Iterate over the other sub-directories
                //transverse_directory(&path)
            } else {
                // TODO : Skip files named index.md | readme.md
                // TODO : Only handle '.md' files
                // Handle this file
                handle_file(&path)
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
                if line.unwrap().contains("toc") {
                    found_toc = true;
                }
                //println!("{}", line.unwrap());
            }
        },
        Err(e) => {
            println!("Error: {:?}", e);
        }
    };

    println!("Found TOC: {}", found_toc);
}


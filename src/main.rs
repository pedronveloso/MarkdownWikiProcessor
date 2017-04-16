use std::fs::{self, File, OpenOptions};
use std::fs::{DirEntry};
use std::io::BufReader;
use std::io::BufRead;
use std::io;
use std::path::Path;


#[allow(dead_code)]
static GEN_TOC: bool = true;

fn main() {
    println!("Generate TOC: {}",GEN_TOC);
    static MAIN_PATH: &'static str = "/home/pedro/documents/markdown_notes";
    let path = Path::new(MAIN_PATH);
    transverse_files(&path);
}

fn transverse_files(path: &Path){
    let entries = fs::read_dir(path).unwrap();

    for entry in entries {
        let path = entry.unwrap().path();
        println!("{}", path.display());
        if path.is_dir() {
            // TODO: Iterate over the other sub-directories
            //transverse_files(&path);
        } else {

        }
    }
}


   /* for entry in paths {
       // println!("Name: {}", path.unwrap().path().display());
        // Locate TOC

        if entry.path().is_dir() {
            println!("New Sub-Dir: {}", entry.unwrap().path().display());
        } else {
            let f = match File::open(entry.unwrap().path().as_path()) {
                Ok(file) => {
                    let br = BufReader::new(file);
                    for line in br.lines() {
                        println!("{}", line.unwrap());
                    }
                },
                Err(e) => {
                    println!("Error: {:?}", e);
                }
            };
        }
    }
}*/


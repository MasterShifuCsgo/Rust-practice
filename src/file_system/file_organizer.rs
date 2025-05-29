/*
file_organizer: Organize files in a directory based on extensions.
Tasks:

List directory contents

Move .txt, .jpg, etc. into their respective folders

Handle missing folders (create if needed)

Use relative and absolute paths

program does not go down nor up the file system.
*/

use std::fs;
use std::path::Path;

const ORGANIZED: &str = "organized"; // specifies where folders with extension names will go. 

pub fn start_file_organizer() {
    //create organized folder
    fs::create_dir_all(ORGANIZED)
        .map_err(|e| format!("Failed to create folder {}\n Error: {}", ORGANIZED, e))
        .expect("failed to create 'organized' folder");

    //for each file in the current directory, create a folder with the same extension name and move it in there.
    for entry in fs::read_dir("./").expect("Failed to read dir") {
        let dir = entry.expect("Result returned Error - DirEntry");

        if dir.path().is_file() {
            // create a folder with the same extension name and move it in there.                        
            
            match dir.path().extension() {
                None => {},
                Some(ext) => {                    
                    println!("{:?}", ext);
                    let p = format!("./{}/{}", ORGANIZED, ext.to_str().unwrap());
                    let target = Path::new(&p);
                    
                    fs::create_dir_all(target).expect("Failed to create folder with specific extension name"); 

                    let new_name = target.join(dir.path());
                    println!("{:?}\n{:?}", dir.path(), new_name);
                                        
                    fs::rename(dir.path(), new_name).expect("Failed to rename file");


                }
            }
        }
    }
}

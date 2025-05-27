/*
file_organizer: Organize files in a directory based on extensions.
Tasks:

List directory contents

Move .txt, .jpg, etc. into their respective folders

Handle missing folders (create if needed)

Use relative and absolute paths

program does not go down nor up the file system.
*/

use std::{
    fmt::{Debug, Formatter},
    fs,
    path::Path,
};

const FOLDER_NAME: &str = "organized";

pub fn start_file_organizer() {
    //create folder organized where all folder with extension names are kept
    fs::create_dir_all(format!("./{}", FOLDER_NAME))
        .expect("Failed to create folder named organized");
    // go through all file names looking at their extension

    if let Ok(entries) = fs::read_dir("./") {
        for entry in entries.flatten() {
            let file_path = entry.path();           

            if !file_path.is_file() {
              continue;
            }

            let ext = match file_path.extension() {
                  Some(e) => e,
                  None => continue
            };
            let folder_path = Path::new(FOLDER_NAME).join(ext);
            if !folder_path.exists() {
                fs::create_dir_all(&folder_path).expect("Failed to create organized folder");
            }

            let new_file_name = folder_path.join(file_path.file_name().expect("Missing file name"));

            println!("Moving file to {:?}", folder_path);

            fs::rename(&file_path, &new_file_name).expect("Failed to create message");
        }
    }
}

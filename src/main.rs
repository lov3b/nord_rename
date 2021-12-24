use std::collections::HashSet;
use std::env;
use std::fs;
use std::fs::DirEntry;

fn main() {
    let this_dir: Vec<DirEntry> = fs::read_dir(env::current_dir().unwrap())
        .unwrap()
        .map(|x| x.unwrap())
        .collect();

    let mut dirs: HashSet<String> = HashSet::new();

    for i in &this_dir {
        if !i.metadata().unwrap().is_dir() {
            let name = i.file_name();
            let name = name.to_str().unwrap();
            let name: String = name.chars().take(2).collect();

            // Make sure not to add dotfiles
            if name.starts_with(".") {
                continue;
            }
            dirs.insert(name);
        }
    }

    // Create directories needed
    for i in &dirs {
        match fs::create_dir(i) {
            Ok(_) => {}
            Err(e) => {
                println!("error, continuing anyways: {:?}", e)
            }
        };
    }
    let current_process = env::current_exe().unwrap();
    let current_process = current_process.file_name().unwrap().to_str().unwrap();
    let current_dir = env::current_dir().unwrap();
    let current_dir = current_dir.file_name().unwrap().to_str().unwrap();
    let current_process = current_process.replace(current_dir, "");

    for i in &this_dir {
        let file = i.file_name();
        let file = file.to_str().unwrap();

        // If the current file == this program, skip
        if file == current_process {
            continue;
        }

        let first_two: String = file.chars().take(2).collect();
        match fs::rename(&file, format!("{}/{}", &first_two, &file)) {
            Ok(_) => {}
            Err(e) => {
                println!("ERROR: {:?}", e);
                std::process::exit(1)
            }
        };
    }
    if &this_dir.len() > &1 {
        println!(
            "moved {} files and created {} directories",
            this_dir.len(),
            &dirs.len()
        );
    } else {
        println!("moved {} file to {} dir", this_dir.len(), &dirs.len());
    }
}

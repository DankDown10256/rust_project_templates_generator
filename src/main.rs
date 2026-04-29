use std::io::{self, Write};
use std::fs;
use std::process::Command;

fn main() {
    println!("Available projects templates :\nFlask\nRust (cargo)\nfrontend (html, css, js)");
    let mut project = String::new();
    print!("Pick one: ");
    io::stdout().flush().unwrap();
    io::stdin()
        .read_line(&mut project)
        .expect("Reading error retry later");
    if project.trim().to_lowercase() == "flask" {
        let mut filename = String::new();
        print!("Project file name : ");
        io::stdout().flush().unwrap();
        io::stdin()
            .read_line(&mut filename)
            .expect("Error in reading retry");
        let filename = filename.trim();
        match fs::create_dir_all(format!("{}/templates", filename)){
            Ok(_) => {
                let files_to_create = vec![
                    format!("{}/app.py", filename),
                    format!("{}/templates/index.html", filename),
                    format!("{}/templates/style.css", filename),
                    format!("{}/templates/app.js", filename),
                ];
                for f in &files_to_create{
                    if let Err(e) = fs::File::create(f) {
                        eprintln!("Error in the creation of {}: {}", f, e);
                    }
                }
                println!("Flask projects available in the directory {}", filename)
            }
            Err(e) => {
                eprintln!("Can't create the template: {}", e);
            }
        }
    }
    else if project.trim().to_lowercase() == "rust" {
        let mut filename = String::new();
        print!("Project file name : ");
        io::stdout().flush().unwrap();
        io::stdin()
            .read_line(&mut filename)
            .expect("Error in reading retry");
        let filename = filename.trim();
        let status = Command::new("cargo")
            .arg("new")
            .arg(filename)
            .status()
            .expect("An error occured");
        if status.success() {
            println!("The rust project have been succefully created with cargo");
        }
        else{
            println!("An error occured");
        }
    }
    if project.trim().to_lowercase() == "frontend" {
        let mut filename = String::new();
        print!("Project file name : ");
        io::stdout().flush().unwrap();
        io::stdin()
            .read_line(&mut filename)
            .expect("Error in reading retry");
        let filename = filename.trim();
        match fs::create_dir_all(format!("{}", filename)){
            Ok(_) => {
                let files_to_create = vec![
                    format!("{}/index.html", filename),
                    format!("{}/style.css", filename),
                    format!("{}/app.js", filename),
                ];
                for f in &files_to_create{
                    if let Err(e) = fs::File::create(f) {
                        eprintln!("Error in the creation of {}: {}", f, e);
                    }
                }
                println!("Frontend template is available in the directory {}", filename)
            }
            Err(e) => {
                eprintln!("Can't create the template: {}", e);
            }
        }
    }
}

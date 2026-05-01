use std::io::{self, Write};
use std::fs;
use std::process::Command;

fn draw_title () {
    let title = "PROJECT TEMPLATES GENERATOR";
    let sub = "Made by DankDown10256 | https://lucas.vincz.fr";
    let width = 100;

    println!("╭{:─^width$}╮", "", width = width);
    println!("│{:^width$}│", title, width = width);
    println!("├{:─^width$}┤", "", width = width);
    println!("│ {:<width$} │", sub, width = width - 2);
    println!("╰{:─^width$}╯", "", width = width);
    println!();
}

fn main() {
    draw_title();
    let mut start = String::new();
    print!("1.Create a new project from a template\n2.Sart an analyze of an existing project(1 or 2): ");
    io::stdout().flush().unwrap();
    io::stdin()
        .read_line(&mut start)
        .expect("An error occured");
    if start.trim() == "1" {
        println!("Available projects templates :\nFlask\nRust (cargo)\nFrontend (html, css, js)\nFlutter\nJava\nIOS");
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
        else if project.trim().to_lowercase() == "frontend" {
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
        else if project.trim().to_lowercase() == "flutter" {
            let mut filename = String::new();
            print!("Project file name : ");
            io::stdout().flush().unwrap();
            io::stdin()
                .read_line(&mut filename)
                .expect("Error in reading retry");
            let filename = filename.trim();
            let flutter_tree = vec![
                ("assets/", true),
                ("lib/", true),
                ("lib/src/", true),
                ("lib/widgets/", true),
                ("lib/main.dart", false),
                ("tests/", true),
                ("pubspec.yaml", false),
            ];
            for (path, is_dir) in flutter_tree {
                let full_path = format!("{}/{}", filename, path);

                if is_dir{
                    if let Err(e) = fs::create_dir_all(&full_path){
                        eprintln!("An error occured retry: {}", e);
                    }
                }
                else{
                    if let Err(e) = fs::File::create(&full_path){
                        eprintln!("Error in the creation of {}: {}", full_path, e);
                    }
                }
            }
        }
        else if project.trim().to_lowercase() == "java" {
            let mut filename = String::new();
            print!("Project file name : ");
            io::stdout().flush().unwrap();
            io::stdin()
                .read_line(&mut filename)
                .expect("Error in reading retry");
            let filename = filename.trim();
            let java_tree = vec![
                ("src/", true),
                ("src/main/", true),
                ("src/main/java/", true),
                ("src/main/java/com/", true),
                ("src/main/java/com/me/", true),
                ("src/main/java/com/me/app/", true),
                ("src/main/java/com/me/app/Main.java", false),
                ("src/main/resources/", true),
                ("target/", true),
                ("pom.xml", false),
            ];
            for (path, is_dir) in java_tree {
                let full_path = format!("{}/{}", filename, path);

                if is_dir{
                    if let Err(e) = fs::create_dir_all(&full_path){
                        eprintln!("An error occured retry: {}", e);
                    }
                }
                else{
                    if let Err(e) = fs::File::create(&full_path){
                        eprintln!("Error in the creation of {}: {}", full_path, e);
                    }
                }
            }
        }
        else if project.trim().to_lowercase() == "ios" {
            let mut filename = String::new();
            print!("Project file name : ");
            io::stdout().flush().unwrap();
            io::stdin()
                .read_line(&mut filename)
                .expect("Error in reading retry");
            let filename = filename.trim();
            let ios_tree: Vec<(String, bool)> = vec![
                ("App/".to_string(), true),
                (format!("App/{}.swift", filename), false),
                ("Views/".to_string(), true),
                ("Views/ContentView.swift".to_string(), false),
                ("Resources/".to_string(), true),
                ("Resources/Assests.xcassets/".to_string(), true),
                ("Resources/Assets.xcassets/Contents.json".to_string(), false),
                ("Resources/Info.plist".to_string(), false),
                ("Tests/".to_string(), true),
                (format!("Tests/{}Tests.swift", filename), false),
            ];
            for (path, is_dir) in ios_tree {
                let full_path = format!("{}/{}", filename, path);

                if is_dir{
                    if let Err(e) = fs::create_dir_all(&full_path){
                        eprintln!("An error occured retry: {}", e);
                    }
                }
                else{
                    if let Err(e) = fs::File::create(&full_path){
                        eprintln!("Error in the creation of {}: {}", full_path, e);
                    }
                }
            }
        }
    }
    else if start.trim() == "2" {
        let mut directory = String::new();
        print!("Which directory to analyze: ");
        io::stdout().flush().unwrap();
        io::stdin()
            .read_line(&mut directory)
            .expect("An error occured");
        let dir = directory.trim();
        let mut tech = String::new();
        print!("Chose in supported technologies for scanning: Flask, Rust, Flutter, IOS, Java, Frontend: ");
        io::stdout().flush().unwrap();
        io::stdin()
            .read_line(&mut tech)
            .expect("An error occured");
        let required_files = match tech.trim().to_lowercase().as_str() {
            "flask" => vec!["app.py", "templates/", "static/", "requirements.txt"],
            "rust" => vec!["Cargo.toml", "src/", "src/main.rs"],
            "frontend" => vec!["index.html", "index.css", "app.js"],
            "flutter" => vec!["assets/", "lib/", "lib/src/", "lib/widgets/", "lib/main.dart", "tests/", "pubspec.yaml"],
            "java" => vec!["src/", "src/main/java/com/me/app/Main.java", "src/main/resources/", "target/", "pom.xml"],
            "ios" => vec!["App/", "Views/", "Views/ContentView.swift", "Resources/", "Resources/Info.plist", "Tests/"],
            _ => vec![]
        };
        if required_files.is_empty() {
            println!("An error occured in the analyze of the directory");
        }
        else {
            println!("Analyze in {}", dir);
            let mut missing_files = 0;
            for item in required_files {
                let full_path = format!("{}/{}", dir, item);
                if std::path::Path::new(&full_path).exists() {
                    println!("The item {} is here", item);
                }
                else {
                    println!("Missing: {}", item);
                    missing_files = missing_files+1;
                }
            }
            if missing_files == 0 {
                println!("Your directory seems to be good");
            }
            else {
                println!("The analyze reported {} missing files/directories", missing_files);
            }
        }
    }
}

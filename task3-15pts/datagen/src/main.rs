use std::{fs::File, io::Write, env, ops::Add};
use rand::distributions::*;

fn generate_file(path: &String, filename: &String, content: &String) {
    std::fs::create_dir_all(path).expect("Path not created");
    let full_path = format!("{path}/{filename}");
    let mut file = File::create(full_path).expect("File not created");
    file.write_all(content.as_bytes()).expect("Data not written to a file");
}

fn generate_random_string(len: usize) -> String {
    return rand::distributions::Alphanumeric.sample_string(&mut rand::thread_rng(), len);
}

fn main() {
    let args: Vec<String> = env::args().collect();
    
    let path: String;
    let number_of_files: usize;

    if args.len() >= 3 {
        path = args[1].clone();
        number_of_files = args[2].parse::<usize>().expect("Size not convertible").clone();
    }
    else {
        println!("Appropriate arguments not provided. Assigning default values (./files/;1000)");
        path = String::from("./files/");
        number_of_files = 1000;
    }

    for i in 0..number_of_files {
        generate_file(&path, &i.to_string().add(".txt"), &generate_random_string(10));
    }

    println!("Generated {number_of_files} files!");
}

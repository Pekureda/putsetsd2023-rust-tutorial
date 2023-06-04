#![allow(dead_code)]

use std::collections::LinkedList;
use std::env;
use std::path::Path;
use rand::Rng;

fn read_directory_files(path: &Path) -> LinkedList<String> {
    let mut result = LinkedList::<String>::new();
    if !path.is_dir() {
        panic!("Provided path is not a directory");
    }

    for entry in std::fs::read_dir(path).expect("Directory is empty") {
        match entry {
            Ok(dir_entry) => {
                let entry_path = dir_entry.path();
                if entry_path.is_file() {
                    result.push_back(String::from(entry_path.to_str().expect("Path is not convertible to a valid UTF-8 string")));
                }
            }
            Err(_) => panic!("File in provided directory is not retrievable"),
        }
    }
    
    return result;
}

fn copy_files_to_directory<'a, T>(iter: &mut T, destination: &Path)
    where T: Iterator<Item = &'a String>
{
    std::fs::create_dir_all(&destination).expect(format!("Destination directory `{}` could not be created", &destination.display()).as_str());

    while let Some(file) = iter.next() {
        let file_path = Path::new(&file);
        let file_name = file_path.file_name().expect("File name not retrievable");
        let destination_file_path = destination.clone().join(file_name);

        std::fs::copy(file_path, &destination_file_path).expect(format!("File {} copy to destination {} has failed", file_path.display(), destination_file_path.display()).as_str());
    }
}

// You can use the function below to randomize index
fn generate_random_number(from: u64, to: u64) -> u64 {
    return rand::thread_rng().gen_range(from..to);
}

fn split_data_set(data_set: &mut LinkedList<String>, percentage_ratio: f64) -> Vec<String> {
    let result = Vec::<String>::new();
    // ####
    // YOUR CODE GOES BELOW
    // todo fill the `result` vec with verification data. Modify data_set in such a way that it will only contain training data.
    // percentage_ratio refers to the percentage of data in data_set that should be in verification data set.



    // YOUR CODE GOES ABOVE
    // ####
    return result;
}

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 3 {
        println!("Invalid number of arguments provided. Expecting `path to data set; percentage of verification set`");
        return;
    }

    let source_directory: String = args[1].clone();
    let percentage_ratio: f64 = args[2].clone().parse::<f64>().expect(format!("Provided argument: `{}` is not convertible to f64", args[2]).as_str());
    let mut data_set = read_directory_files(Path::new(&source_directory));

    let verification_data_set = split_data_set(&mut data_set, percentage_ratio);
    let training_data_set = data_set;

    copy_files_to_directory(&mut verification_data_set.iter(), Path::new("./verification"));
    copy_files_to_directory(&mut training_data_set.iter(), Path::new("./training"));

    println!("Check your results :)");
}

use std::env;
use walkdir::WalkDir;
use std::fs;

fn help() {
    println!("
usage:\t shred-rs <file/directory>
    
Shred a given file, or files within a given directory.");
}

fn do_the_work(passed_arg: &String) {
    for files in WalkDir::new(passed_arg)
    .follow_links(true)
    .into_iter()
    .filter_map(|e| e.ok()) {
        if files.metadata().unwrap().is_file() {
            let file = files.path();
            // let file_metadata = fs::metadata(filez).unwrap();
            // let file_size: usize = file_metadata.len().try_into().unwrap();
            let file_size: usize = files.metadata().unwrap().len().try_into().unwrap(); // one liner for the above 2 lines.
            fs::write(file, vec![0; file_size]).expect("Something went wrong while overwriting!"); // shred files with all 0s!
        }
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();

    match args.len() {
        
        1 => { // no arguments passed
            help();
        },
        
        2 => { // one argument passed
            let passed_arg = &args[1];
            do_the_work(passed_arg);
        },
        
        _ => { // show help on all the other cases
            help();
        }
    }
}

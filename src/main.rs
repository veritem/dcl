use std::fs;

fn main() {
    print_files();
}

fn print_files() {
    let paths = fs::read_dir("./").unwrap();

    for path in paths {
        println!("Name: {}", path.unwrap().path().display());
    }
}

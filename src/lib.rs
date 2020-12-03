use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}

pub fn get_input(file_path: &str) -> String {
    let path = Path::new(file_path);
    let mut file = File::open(&path).expect("Error opening input file!");

    let mut input = String::new();
    file.read_to_string(&mut input).expect("Error reading input file!");

    input
}

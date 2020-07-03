use std::fs::File;
use std::io::ErrorKind;

fn main() {
    let f = File::open("hello.txt").expect("Failed to open");
    let f = File::open("hello.txt").unwrap_or_else(|error| {
        if error.kind() == ErrorKind::NotFound {
            File::create("hello.txt").unwrap_or_else(|error| {
                panic!("problem creating {:?}", error);
            })
        } else {
            panic!("problem opening {:?}", error)
        }
    });
}

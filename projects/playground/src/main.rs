use std::fs::File;
use std::io::ErrorKind;

fn main() {
    let f = File::open("hello.txt");
    let f = match f {
        Ok(file) => file,
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create("hello.txt") {
                Ok(fc) => fc,
                Err(e) => panic!("problem creating {:?}", e),
            },
            other_error => {
                panic!("problem opening {:?}", other_error)
            }
        },
    };
}

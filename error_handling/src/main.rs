use std::fs::File;
use std::io::{self, Read, ErrorKind};
fn read_stuff() -> Result<String, io::Error> {
 // {
    //     let mut username_file = File::open("hello.txt")?;
    //     let mut username = String::new();
    //     username_file.read_to_string(&mut username)?;
    //     Ok(username)
    // }
    {
        let mut username = String::new();
        File::open("hello.txt")?.read_to_string(&mut username)?;
        Ok(username)
    }
}
fn main() {
    // {
    //     let file = File::open("hello.txt");
    //     let hello_file = match file {
    //         Ok(file) => file,
    //         Err(error) => match error.kind() {
    //             ErrorKind::NotFound => match File::create("hello.txt") {
    //                 Ok(fc) => fc,
    //                 Err(err) => panic!("Problem creating the file {:?}", err),
    //             },
    //             other_error => panic!("Problem opening the file {:?}", other_error),
    //         },
    //     };
    // }
    // {
    //     let greeting_file =
    //         File::open("hello.txt").expect("hello.txt should be included in this project");
    // }
   read_stuff();
}

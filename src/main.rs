mod syntax;
mod error;
mod front;
mod preproc;

use std::{
    fs::File,
    io::prelude::*,
    path::Path,
};

use front::Lexer;

fn main() {
    let path = Path::new("D:\\Projects\\cebu\\tests\\basic.cebu");
    let display = path.display();
    let mut file = match File::open(&path) {
        Err(why) => panic!("Could not open {}: {}", display, why),
        Ok(file) => file,
    };

    let mut src = String::new();
    match file.read_to_string(&mut src) {
        Err(why) => panic!("Could not read {}:\n{}", display, why),
        Ok(_) => {
            match &mut Lexer::new(src.as_str()) {
                Ok(lexer) => {
                    loop {
                        match lexer.get_next_token() {
                            Ok(t) => println!("{:?}", t),
                            Err(e) => { println!("{:?}", e); break; },
                        }
                    }
                },
                Err(_) => todo!(),
            }
        }
    }
}
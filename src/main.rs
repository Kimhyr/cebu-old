mod syntax;
mod front;

use std::{
    fs::File,
    io::prelude::*,
    path::Path,
};


fn main() {
    let path = Path::new("D:\\Projects\\cebu\\tests\\basic.cebu");
    let display = path.display();
    let mut file = match File::open(&path) {
        Err(why) => panic!("Could not open {}: {}", display, why),
        Ok(file) => file,
    };

    let mut src = String::new();
    match file.read_to_string(&mut src) {
        Err(why) => panic!("Could not read {}: {}", display, why),
        Ok(_) => {
            compile(src.as_str());
        }
    }
}

fn compile<'a>(src: &'a str) {

}
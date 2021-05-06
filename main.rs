use std::env::args;
use std::io::{self, Write};


fn main()  {
    let mut files = <Vec<String>>::new();
    let pattern = std::env::args().nth(1).expect("No pattern");
    for parsing in args().skip(2){
        files.push(parsing);
    }
    for path in paths{
        let text = std::fs::read_to_string(&files).expect("Not a valid file");
        for line in text.lines(){
            if line.contains(&pattern){
                println!("{:?}", line);
            }
        }
    }
}

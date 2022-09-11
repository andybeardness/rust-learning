use std::env;
use std::fs::File;
use std::io::Read;
use std::io;

fn file1() {
    let first = 
        env::args()
        .nth(1)
        .expect("no filename");

    let mut file = 
        File::open(&first)
        .expect("can't open file");

    let mut text = String::new();

    file.read_to_string(&mut text)
        .expect("can't read file");
    
    println!("file had {} bytes", text.len());
}

fn good_or_bad(good: bool) -> Result<i32, String> {
    if good {
        Ok(42)
    } else {
        Err("bad".to_string())
    }
}

fn file2() {
    println!("{:?}", good_or_bad(true));
    println!("{:?}", good_or_bad(false));

    match good_or_bad(true) {
        Ok(n) => println!("it's good '{}'", n),
        Err(msg) => println!("oops, error '{}'", msg),
    }
}

fn read_to_string(filename: &str) -> Result<String, io::Error> {
   let mut file = match File::open(&filename) {
        Ok(f) => f,
        Err(e) => return Err(e),
   }; 

   let mut text = String::new();

   match file.read_to_string(&mut text) {
       Ok(_) => Ok(text),
       Err(e) => Err(e),
       
   }
}

fn read_to_string2(filename: &str) -> io::Result<String> {
    let mut file = File::open(&filename)?;
    let mut text = String::new();
    file.read_to_string(&mut text);
    Ok(text)
}

fn read_to_string3(filename: &str) -> io::Result<String> {
    let mut file = try!(File::open(&filename));
    let mut text = String::new();
    file.read_to_string(&mut text);
    Ok(text)
}

fn file3() {
    let file = env::args()
        .nth(1)
        .expect("can't read file");

    let text = read_to_string(&file)
        .expect("can't get text from file");

    println!("file had {} bytes", text.len());
}

fn main() {
    file1();
    file2();
    file3();
}

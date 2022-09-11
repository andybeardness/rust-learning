fn match1() {
    let mut word = String::new();
    word.push_str("hello world!");
    
    match word.find('w') {
        Some(idx) => {
            let hi = &word[idx..];
            println!("hi is '{}'", hi);
        },
        None => println!("none branch"),
    };

}

fn match2() {
    let word = "hello world".to_string();

    if let Some(idx) = word.find('w') {
        println!("word is '{}'", &word[idx..]);
    }
}

fn match3() {
    let n = 4;

    let text = match n {
        0 => "hello",
        1 => "world",
        2 => "is",
        3 => "my",
        4 => "home",
        5 => "nose",
        _ => "other",
    };

    println!("text == {}", text);
}

fn match4() {
    let n = 4;

    let text = match n {
        0..=3 => "text",
        4..=6 => "home",
        _ => "other",
    };

    println!("text is {}", text);

}

fn main() {
   match1(); 
   match2();
   match3();
   match4();
}

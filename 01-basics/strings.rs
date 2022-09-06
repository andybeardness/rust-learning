fn str1() {
    let text = "hello world";
    let s = text.to_string();

    println!("text is {}", text);
    println!("s is {}", s);
}

fn str2() {
    let mut s = String::new();

    s.push('H');
    s.push_str("ello");
    s.push(' ');
    s += "World!";
    s.pop();

    println!("s : {}", s);
}

fn array_to_str(array: &[i32]) -> String {
    let mut s = String::new();

    s.push('[');
    
    for v in array {
       s += &v.to_string();
       s.push(',');
    }

    s.pop();
    s.push(']');
    s
}

fn str_format() {
    let arr = [10, 20, 30];
    let slice = &arr;
    
    let string_array = array_to_str(slice);
    let format_array = format!("hello {}", string_array);

    assert_eq!(format_array, "hello [10,20,30]");
    println!("format_array is {}", format_array);
}

fn str_slice() {
    let text  = "text";
    let s = "string".to_string();

    let text_slice = &text[1..3];
    let string_slice = &s[1..3];

    println!("text_slice   = {}", text_slice);
    println!("string_slice = {}", string_slice);
}

fn str_split_whites() {
    let text = "text with white spaces for tests";
    let words: Vec<&str> = text.split_whitespace().collect();

    println!("words = {:?}", words);
}

fn str_filter() {
    let text = "hello world i said them today maybe you warcraft";
    let stripped: String = text
        .chars()
        .filter(|ch| !ch.is_whitespace())
        .collect();

    println!("stripped == {}", stripped);
}

fn main() {
    str1();
    str2();
    str_format();
    str_slice();
    str_split_whites();
    str_filter();
}

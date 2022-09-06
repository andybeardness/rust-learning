fn safe_get() {
    let ints = [1, 2, 3, 4, 5, 6];
    let slice = &ints;
    let first = slice.get(0);
    let sixth = slice.get(6);

    println!("first : {:?}", first);
    println!("sixth : {:?}", sixth);

    let first_is_some = first.is_some();
    let first_is_none = first.is_none();

    let sixth_is_some = sixth.is_some();
    let sixth_is_none = sixth.is_none();

    let first_unwrap = first.unwrap();

    println!("first_is_some : {}", first_is_some);
    println!("first_is_none : {}", first_is_none);
    
    println!("sixth_is_some : {}", sixth_is_some);
    println!("sixth_is_none : {}", sixth_is_none);

    println!("first_unwrap  : {}", first_unwrap);
}

fn main() {
   safe_get(); 
}

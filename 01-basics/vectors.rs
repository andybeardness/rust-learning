fn vectors() {
    let mut vec = Vec::new();
    vec.push(10);
    vec.push(20);
    vec.push(30);
    vec.push(40);

    let first = vec[0];
    let maybe_first = vec.get(0);
    
    println!("vec         : {:?}", vec);
    println!("first       : {}", first);
    println!("maybe_first : {:?}", maybe_first);
}

fn dump(slice: &[i32]) {
    println!("slice is {:?}", slice);
}

fn verctors_slice() {
    let mut vec = Vec::new();
    vec.push(10);
    vec.push(20);
    vec.push(30);

    dump(&vec);

    let slice = &vec[1..];

    println!("vec slice is {:?}", slice);
}

fn main() {
    vectors();
    verctors_slice();
}

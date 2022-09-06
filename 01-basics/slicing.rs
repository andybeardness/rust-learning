fn pretty_print() {
    let ints = [1, 2, 3, 4, 5];
    let floats = [1.1, 2.2, 3.3, 4.4];
    let strings = ["hello", "world"];
    let ints_ints = [[1, 2], [3, 4]];

    println!("ints {:?}", ints);
    println!("floats {:?}", floats);
    println!("strings {:?}", strings);
    println!("ints_ints {:?}", ints_ints);
}

fn slice() {
    let ints = [1, 2, 3, 4, 5];
    let slice1 = &ints[0..2];
    let slice2 = &ints[2..];

    println!("ints {:?}", ints);
    println!("slice1 {:?}", slice1);
    println!("slice2 {:?}", slice2);
}

fn main() {
    pretty_print();
    slice();
}

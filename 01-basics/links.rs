fn by_ref(x: &i32) -> i32 {
    *x + 1
}

fn modifies(x: &mut f64) {
    *x = 1.0;
}

fn main() {
    let i = 10;
    let res1 = by_ref(&i);
    let res2 = by_ref(&42);
    
    println!("res1 == {}, res2 == {}", res1, res2);
    println!("i == {}", i);

    let mut a = 101.101;
    println!("a == {}", a);
    modifies(&mut a);
    println!("a == {}", a);
}

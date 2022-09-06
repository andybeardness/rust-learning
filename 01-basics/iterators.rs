fn default_iter() {
    println!("default_iter");

    let mut iter = 0..3;

    assert_eq!(iter.next(), Some(0));
    assert_eq!(iter.next(), Some(1));
    assert_eq!(iter.next(), Some(2));
    assert_eq!(iter.next(), None);
}

fn iter2() {
    println!("iter2");

    let arr = [1, 2, 3, 4, 5];

    for i in arr {
        println!("i == {}", i);
    }
}

fn iter3() {
    println!("iter3");

    let arr = [1, 2, 3, 4];
    let iter = arr.iter();

    for i in iter {
        println!("i == {}", i);
    }
}

fn iter4() {
    println!("iter4");

    let arr = [1, 2, 3, 4];
    let slice = &arr;

    for i in slice {
        println!("i == {}", i);
    }
}

fn cool_sum() {
    println!("cool_sum");

    let sum: i32 = (0..100).sum();
    println!("sum is {}", sum);

    let sum: i64 = [10, 20, 30, 40].iter().sum();
    println!("sum is {}", sum);
}

fn windows() {
    println!("windows");

    let ints = [1, 2, 3, 4, 5, 6, 7];
    let slice = &ints;

    for w in slice.windows(2) {
        println!("w == {:?}", w);
    }
}

fn chunks() {
    println!("chunks");

    let ints = [1, 2, 3, 4, 5];
    let slice = &ints;

    for chunk in slice.chunks(2) {
        println!("chunk == {:?}", chunk);
    }
}

fn main() {
    default_iter();
    iter2();
    iter3();
    iter4();
    cool_sum();
    windows();
    chunks();
}

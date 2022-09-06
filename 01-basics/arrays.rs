fn arrays() {
    let arr = [10, 20, 30, 40];
    let first = arr[0];

    println!("first {}", first);

    for i in 0..4 {
        println!("arr[{}] = {}", i, arr[i])
    }

    println!("len == {}", arr.len());
}

fn sum_of_slice(values: &[i32]) -> i32 {
    let mut sum = 0;

    for i in 0..values.len() {
        sum += values[i];
    }

    sum
}

fn arrays2() {
    let arr = [10, 20, 30, 40];

    let sum = sum_of_slice(&arr);

    println!("sum == {}", sum);
}

fn main() {
    arrays();
    arrays2();
}

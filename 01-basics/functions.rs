fn i_sqr(x: i32) -> i32 {
    return x * x;
}

fn f_sqr(x: f64) -> f64 {
    x * x
}

fn abs(x: f64) -> f64 {
    if x >= 0.0 { x } 
    else { -x }
}

fn clamp(x: f64, x1: f64, x2: f64) -> f64 {
    if x < x1 { 
        x1 
    } else if x > x2 {
        x2
    } else {
        x
    }
}

fn factorial(n: u64) -> u64 {
    if n == 1 {
        1
    } else {
        n * factorial(n - 1)
    }
}

fn main() {
    let res = i_sqr(14);
    let fres = f_sqr(19.0);

    println!("res  is {}", res);
    println!("fres is {}", fres);

    let abs_one = abs(100.0);
    let abs_two = abs(-999.0);

    println!("abs_one is {}", abs_one);
    println!("abs_two is {}", abs_two);

    let clamp_one = clamp(2.0, 5.0, 2.0);
    let clamp_two = clamp(10.0, 5.0, 8.0);
    let clamp_three = clamp(10.0, 2.0, 20.0);

    println!("clamp_one is {}", clamp_one);
    println!("clamp_two is {}", clamp_two);
    println!("clamp_three is {}", clamp_three);

    let factorial_11 = factorial(11);
    let factorial_12 = factorial(12);
    let factorial_13 = factorial(13);

    println!("factorial_11 is {}", factorial_11);
    println!("factorial_12 is {}", factorial_12);
    println!("factorial_13 is {}", factorial_13);
}

fn one() {
    let pi: f64 = 3.1415;
    let x = pi / 2.0;
    let x_cos = x.cos();

    println!("pi = {}, x = {}, x_cos = {}", pi, x, x_cos);
}

fn two() {
    let pi = std::f64::consts::PI;
    let x = pi / 2.0;
    let x_cos = x.cos(); 
    
    println!("pi = {}, x = {}, cos = {}", pi, x, x_cos);
}

fn main() {
    one();
    two();
}

fn main() {
    for i in 0..12 {
        if i % 2 == 0 {
            println!("i == {}, it's even", i);
        } else {
            println!("i == {}, it's odd", i)
        }
    }

    for i in 0..12 {
        let number_type = 
            if i % 2 == 0 { "even" }
            else { "odd" };

        println!("{} {}", number_type, i);
    }
}

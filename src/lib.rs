mod generator;

pub fn print_random_number() {
    let n = generator::get_ran();
    println!("Random u8: {}", n);
}
use rand::{generate_rand_num, generate_rand_num_range};

mod rand;

fn main() {
    generate_rand_num();

    println!("------------------");

    generate_rand_num_range();
}

use rand::{
    gen_password_from_custom_chars, generate_normal_sample, generate_rand_num,
    generate_rand_num_range, generate_rand_password_from_chars, generate_random_custom_type,
};
use rand_distr::NormalError;

mod rand;

fn main() -> Result<(), NormalError> {
    generate_rand_num();

    println!("------------------");

    generate_rand_num_range();

    println!("------------------");

    // 生成一个均值为 2.0，标准差为 3.0 的正态分布样本
    generate_normal_sample(2.0, 3.0)?;

    println!("------------------");

    generate_random_custom_type();

    println!("------------------");

    generate_rand_password_from_chars();

    println!("------------------");

    gen_password_from_custom_chars();

    Ok(())
}

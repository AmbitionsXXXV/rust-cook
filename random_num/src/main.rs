use rand::{generate_rand_num, generate_rand_num_range};
use rand_distr::NormalError;

use crate::rand::{generate_normal_sample, generate_random_custom_type};

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

    Ok(())
}

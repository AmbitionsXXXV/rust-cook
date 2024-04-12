use rand::{
    distributions::{Distribution, Uniform},
    Rng,
};

pub fn generate_rand_num() {
    let mut rng = rand::thread_rng();

    let n1: u8 = rng.gen();
    let n2: u16 = rng.gen();

    println!("Random u8: {}", n1);
    println!("Random u16: {}", n2);
    println!("Random u32: {}", rng.gen::<u32>());
    println!("Random i32: {}", rng.gen::<i32>());
    println!("Random float: {}", rng.gen::<f64>());
}

pub fn generate_rand_num_range() {
    let mut rng = rand::thread_rng();

    // 生成一个范围在左闭右开区间的随机数
    // println!("Integer: {}", rng.gen_range(0..10));
    // println!("Float: {}", rng.gen_range(0.0..10.0));

    // 使用 Uniform 模块可以得到均匀分布的值。下述代码和上述代码具有相同的效果，
    // 但在相同范围内重复生成数字时，下述代码性能可能会更好
    let die = Uniform::from(1..10);

    loop {
        let throw = die.sample(&mut rng);
        println!("Roll the die: {}", throw);
        if throw == 9 {
            break;
        }
    }
}

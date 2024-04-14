use rand::{
    distributions::{Distribution, Uniform},
    thread_rng, Rng,
};
use rand_distr::{Normal, NormalError, Standard};

pub fn generate_rand_num() {
    let mut rng = thread_rng();

    let n1: u8 = rng.gen();
    let n2: u16 = rng.gen();

    println!("Random u8: {}", n1);
    println!("Random u16: {}", n2);
    println!("Random u32: {}", rng.gen::<u32>());
    println!("Random i32: {}", rng.gen::<i32>());
    println!("Random float: {}", rng.gen::<f64>());
}

pub fn generate_rand_num_range() {
    let mut rng = thread_rng();

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

pub fn generate_normal_sample(mean: f64, std_dev: f64) -> Result<(), NormalError> {
    let mut rng = thread_rng();
    let normal = Normal::new(mean, std_dev)?;
    let sample = normal.sample(&mut rng);

    println!("{} is from a N({}, {})", sample, mean, std_dev);

    Ok(())
}

#[derive(Debug)]
struct Point {
    x: i32,
    y: i32,
}

// 随机生成一个元组 (i32, bool, f64) 和用户定义类型为 Point 的变量。
// 为 Standard 实现 Distribution trait，以允许随机生成
impl Distribution<Point> for Standard {
    /// 从标准随机数生成器中生成一个随机的点。
    ///
    /// # 参数
    ///
    /// - `rng`：随机数生成器的引用。
    ///
    /// # 返回值
    ///
    /// 返回一个随机生成的点。
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Point {
        let (rand_x, rand_y) = rng.gen();

        Point {
            x: rand_x,
            y: rand_y,
        }
    }
}

pub fn generate_random_custom_type() {
    let mut rng = rand::thread_rng();
    let rand_tuple = rng.gen::<(i32, bool, f64)>();
    let rand_point: Point = rng.gen();

    println!("Random tuple: {:?}", rand_tuple);
    println!("Random point.x: {}", rand_point.x);
    println!("Random point.y: {}", rand_point.y);
}

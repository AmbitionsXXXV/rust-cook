use rand::{
    distributions::{Distribution, Uniform},
    thread_rng, Rng,
};
use rand_distr::{Alphanumeric, Normal, NormalError, Standard};

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

/// 生成一个随机的 30 字符密码
/// 随机生成一个给定长度的 ASCII 字符串，范围为 A-Z，a-z，0-9，使用字母数字样本
pub fn generate_rand_password_from_chars() {
    // 创建一个随机数生成器
    let rand_string = thread_rng()
        // 从 Alphanumeric 分布中采样字符。Alphanumeric 是一个产生包含大写字母、小写字母和数字的字符的分布
        .sample_iter(&Alphanumeric)
        // 设置生成字符串的长度为 30 个字符
        .take(30)
        // 将生成的随机字节转换为字符
        .map(char::from)
        // 将迭代器中的字符收集成一个字符串
        .collect::<String>();

    // 输出生成的随机字符串
    println!("Random string: {}", rand_string);
}

// 使用用户自定义的字节字符串，使用 gen_range 函数，随机生成一个给定长度的 ASCII 字符串
/// 从自定义字符集生成指定长度的随机密码
pub fn gen_password_from_custom_chars() {
    // 定义可用于生成密码的字符集
    const CHARSET: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZ\
                            abcdefghijklmnopqrstuvwxyz\
                            0123456789)(*&^%$#@!~";

    // 定义生成密码的长度
    const PASSWORD_LEN: usize = 30;

    // 创建一个随机数生成器
    let mut rng = rand::thread_rng();

    // 生成一个随机密码
    let password = (0..PASSWORD_LEN)
        .map(|_| {
            // 随机选择一个索引位置
            let idx = rng.gen_range(0..CHARSET.len());
            // 根据索引位置从字符集中选择字符，并转换为 char 类型
            CHARSET[idx] as char
        })
        .collect::<String>(); // 将迭代器中的字符收集成一个完整的字符串

    // 打印生成的密码
    println!("{:?}", password);
}

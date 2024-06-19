// src/lib.rs
use rayon::prelude::*;

pub fn serial_change_arr_ele(arr: &mut [i32]) {
    for p in arr.iter_mut() {
        *p -= 1;
    }
}

/// 并行地将数组中的每个元素减 1。
///
/// # 参数
///
/// * `arr` - 一个可变引用，指向要处理的 i32 数组。
///
/// # 示例
///
/// ```
/// use crate::parallel::parallel_change_arr_ele;
///
/// let mut arr = [0, 7, 9, 11];
/// parallel_change_arr_ele(&mut arr);
/// assert_eq!(arr, [-1, 6, 8, 10]);
/// ```
pub fn parallel_change_arr_ele(arr: &mut [i32]) {
    // 使用 Rayon 并行处理数组，将每个元素减 1
    arr.par_iter_mut().for_each(|p| *p -= 1)
}

// 测试模块
#[cfg(test)]
mod tests {
    use super::*;
    use std::time::Instant;

    /// 测试 `parallel_change_arr_ele` 函数
    #[test]
    fn it_works() {
        // 初始化一个数组
        let mut arr = [1, 25, -4, 10];

        // 调用 `parallel_change_arr_ele` 函数
        parallel_change_arr_ele(&mut arr);

        // 检查数组元素是否按预期减少了 1
        assert_eq!(arr, [0, 24, -5, 9]);
    }

    /// 测试串行和并行处理的性能
    /// 这个数量下，并行的效率大概是串行的 2.3 倍
    #[test]
    fn test_performance() {
        // 初始化一个大型数组
        let mut large_array: Vec<i32> = (0..1_000_000).collect();

        // 测试串行处理时间
        let start = Instant::now();
        serial_change_arr_ele(&mut large_array);
        let duration_serial = start.elapsed();
        println!("串行处理时间: {:?}", duration_serial);

        // 重置数组
        let mut large_array: Vec<i32> = (0..1_000_000).collect();

        // 测试并行处理时间
        let start = Instant::now();
        parallel_change_arr_ele(&mut large_array);
        let duration_parallel = start.elapsed();
        println!("并行处理时间: {:?}", duration_parallel);
    }
}

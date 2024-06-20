use rayon::prelude::*;

/// 并行地检查数组中的元素是否符合任意条件。
///
/// # 参数
///
/// * `vec` - 一个引用，指向要处理的 i32 数组。
/// * `condition` - 一个闭包，用于检查条件。
///
/// # 返回值
///
/// 如果数组中有任意元素符合条件，则返回 `true`，否则返回 `false`。
///
/// # 示例
///
/// ```
/// use crate::parallel::parallel_check_conditions::parallel_any_condition;
///
/// let vec = vec![2, 4, 6, 8];
///
/// assert!(!parallel_any_condition(&vec, |n| (*n % 2) != 0));
/// assert!(parallel_any_condition(&vec, |n| (*n % 2) == 0));
/// assert!(!parallel_any_condition(&vec, |n| *n > 8 ));
/// assert!(parallel_any_condition(&vec, |n| *n <= 8 ));
///
/// let vec_with_odd = vec![2, 4, 6, 8, 9];
///
/// assert!(parallel_any_condition(&vec_with_odd, |n| (*n % 2) != 0));
/// ```
pub fn parallel_any_condition<F>(vec: &[i32], condition: F) -> bool
where
    F: Fn(&i32) -> bool + Sync,
{
    vec.par_iter().any(|&n| condition(&n))
}

/// 并行地检查数组中的所有元素是否符合条件。
///
/// # 参数
///
/// * `vec` - 一个引用，指向要处理的 i32 数组。
/// * `condition` - 一个闭包，用于检查条件。
///
/// # 返回值
///
/// 如果数组中的所有元素都符合条件，则返回 `true`，否则返回 `false`。
///
/// # 示例
///
/// ```
/// use crate::parallel::parallel_check_conditions::parallel_all_condition;
///
/// let vec = vec![2, 4, 6, 8];
///
/// assert!(parallel_all_condition(&vec, |n| (*n % 2) == 0));
/// assert!(!parallel_all_condition(&vec, |n| (*n % 2) != 0));
/// assert!(parallel_all_condition(&vec, |n| *n <= 8 ));
/// assert!(!parallel_all_condition(&vec, |n| *n > 8 ));
///
/// let vec_with_odd = vec![2, 4, 6, 8, 9];
///
/// assert!(!parallel_all_condition(&vec_with_odd, |n| (*n % 2) == 0));
/// ```
pub fn parallel_all_condition<F>(vec: &[i32], condition: F) -> bool
where
    F: Fn(&i32) -> bool + Sync,
{
    vec.par_iter().all(|&n| condition(&n))
}

// 测试模块
#[cfg(test)]
mod tests {
    use super::*;

    /// 测试 `parallel_any_condition` 和 `parallel_all_condition` 函数
    #[test]
    fn it_works() {
        let mut vec = vec![2, 4, 6, 8];

        assert!(!parallel_any_condition(&vec, |n| (*n % 2) != 0));
        assert!(parallel_all_condition(&vec, |n| (*n % 2) == 0));
        assert!(!parallel_any_condition(&vec, |n| *n > 8));
        assert!(parallel_all_condition(&vec, |n| *n <= 8));

        vec.push(9);

        assert!(parallel_any_condition(&vec, |n| (*n % 2) != 0));
        assert!(!parallel_all_condition(&vec, |n| (*n % 2) == 0));
        assert!(parallel_any_condition(&vec, |n| *n > 8));
        assert!(!parallel_all_condition(&vec, |n| *n <= 8));
    }

    /// 测试并行处理的性能
    #[test]
    fn test_performance() {
        let large_array: Vec<i32> = (0..1_000_000).collect();

        // 测试并行检查 any 条件的时间
        let start = std::time::Instant::now();
        parallel_any_condition(&large_array, |n| *n > 500_000);
        let duration_any = start.elapsed();
        println!("并行检查 any 条件时间: {duration_any:#?}");

        // 测试并行检查 all 条件的时间
        let start = std::time::Instant::now();
        parallel_all_condition(&large_array, |n| *n <= 1_000_000);
        let duration_all = start.elapsed();
        println!("并行检查 all 条件时间: {duration_all:#?}");
    }
}

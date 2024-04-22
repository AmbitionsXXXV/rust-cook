use crossbeam::scope;

pub fn generate_short_term_threads() {
    let arr = &[1, 25, -4, 10];
    let max = find_max(arr);

    match max {
        Some(value) => println!("The maximum value is {}", value),
        None => println!("The array is empty"),
    }
}

fn find_max(arr: &[i32]) -> Option<i32> {
    const THRESHOLD: usize = 2; // 设定一个阈值，小于等于这个值时直接在当前线程计算最大值

    // 如果数组长度小于或等于阈值，直接计算最大值
    if arr.len() <= THRESHOLD {
        return arr.iter().cloned().max();
    }

    // 二分法分割数组
    let mid = arr.len() / 2; // 找到数组的中点
    let (left, right) = arr.split_at(mid); // 将数组分为左右两部分

    // 通过 crossbeam 的 scope 创建线程，避免生命周期问题
    // 在 scope 中所有通过 s.spawn 创建的线程都会在 scope 结束时自动 join，避免线程泄漏
    scope(|s| {
        let thread_l = s.spawn(|_| find_max(left)); // 在新线程中递归计算左半数组的最大值
        let thread_r = s.spawn(|_| find_max(right)); // 在新线程中递归计算右半数组的最大值

        // 等待左侧线程完成，并处理可能的错误
        let max_l = thread_l.join().unwrap()?;
        // 等待右侧线程完成，并处理可能的错误
        let max_r = thread_r.join().unwrap()?;

        Some(max_l.max(max_r)) // 返回两个子结果中的较大值
    })
    .unwrap() // 处理 scope 可能的错误
}

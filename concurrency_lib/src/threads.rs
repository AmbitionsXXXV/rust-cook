use std::{thread, time::Duration};

use crossbeam::{channel::bounded, scope};

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

// 下面的实例使用 crossbeam 和 crossbeam-channel 两个 crate 创建了一个并行的管道，与 ZeroMQ 指南 中所描述的类似：
// 管道有一个数据源和一个数据接收器，数据在从源到接收器的过程中由两个工作线程并行处理。

// 我们使用容量由 crossbeam_channel::bounded 分配的有界信道。生产者必须在它自己的线程上，因为它产生的消息比
// 工作线程处理它们的速度快（因为工作线程休眠了半秒）——这意味着生产者将在对 [crossbeam_channel::Sender::send] 调用时阻塞半秒，
// 直到其中一个工作线程对信道中的数据处理完毕。也请注意，信道中的数据由最先接收它的任何工作线程调用，因此每个消息都传递给单个工作线程，而不是传递给两个工作线程。

// 通过迭代器 crossbeam_channel::Receiver::iter 方法从信道读取数据，这将会造成阻塞，要么等待新消息，要么直到信道关闭。
// 因为信道是在 crossbeam::scope 范围内创建的，我们必须通过 drop 手动关闭它们，以防止整个程序阻塞工作线程的 for 循环。
// 你可以将对 drop 的调用视作不再发送消息的信号。
pub fn create_concurrent_pipelines() {
    // 创建两个有界信道
    let (snd1, rcv1) = bounded(1);
    let (snd2, rcv2) = bounded(1);

    // 定义消息数量和工作线程数量
    let n_msgs = 4;
    let n_workers = 2;

    // 使用 crossbeam 的作用域来控制线程的生命周期
    crossbeam::scope(|s| {
        // 生产者线程：发送一定数量的消息
        s.spawn(|_| {
            for i in 0..n_msgs {
                snd1.send(i).unwrap();
                println!("Source sent {}", i);
            }

            // 发送完毕后关闭第一个信道
            drop(snd1);
        });

        // 创建两个工作线程处理信道中的消息
        for _ in 0..n_workers {
            let (sender, receiver) = (snd2.clone(), rcv1.clone());

            s.spawn(move |_| {
                thread::sleep(Duration::from_millis(500)); // 模拟处理延时

                for msg in receiver.iter() {
                    println!("Worker {:?} received {}.", thread::current().id(), msg);
                    sender.send(msg * 2).unwrap(); // 加工消息后发送
                }
            });
        }

        // 关闭第二个信道，以便接收器知道何时停止
        drop(snd2);

        // 接收器：接收所有工作线程处理后的消息
        for msg in rcv2.iter() {
            println!("Sink received {}", msg);
        }
    })
    .unwrap(); // 确保处理作用域内的错误
}

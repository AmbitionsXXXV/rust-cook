#[test]
fn test_generate_short_term_threads() {
    // 由于这个函数只是打印输出,我们可以测试它不会崩溃
    use crate::generate_short_term_threads;

    generate_short_term_threads();
}

#[test]
fn test_create_concurrent_pipelines() {
    // 同样,我们主要测试这个函数不会崩溃
    use crate::create_concurrent_pipelines;

    create_concurrent_pipelines();
}

#[test]
fn test_passing_data_between_threads() {
    // 同样,我们主要测试这个函数不会崩溃
    use crate::passing_data_between_threads;

    passing_data_between_threads();
}

#[test]
fn test_create_concurrent_pipelines_message_count() {
    use crossbeam::channel::{bounded, unbounded};
    use crossbeam::scope;
    use std::sync::atomic::{AtomicUsize, Ordering};
    use std::sync::Arc;
    use std::time::Duration;

    let (snd1, rcv1) = bounded(1);
    let (snd2, rcv2) = unbounded();

    let n_msgs = 100;
    let n_workers = 4;
    let counter = Arc::new(AtomicUsize::new(0));

    scope(|s| {
        s.spawn(|_| {
            for i in 0..n_msgs {
                snd1.send(i).unwrap();
            }
            drop(snd1);
        });

        for _ in 0..n_workers {
            let (sender, receiver) = (snd2.clone(), rcv1.clone());
            let counter = counter.clone();

            s.spawn(move |_| {
                for msg in receiver.iter() {
                    std::thread::sleep(Duration::from_millis(10)); // 模拟处理时间
                    sender.send(msg * 2).unwrap();
                    counter.fetch_add(1, Ordering::SeqCst);
                }
            });
        }

        drop(snd2);

        let mut received = 0;
        for _ in rcv2.iter() {
            received += 1;
        }

        assert_eq!(received, n_msgs);
        assert_eq!(counter.load(Ordering::SeqCst), n_msgs);
    })
    .unwrap();
}

#[test]
fn test_passing_data_between_threads_order() {
    use crossbeam::channel::unbounded;
    use crossbeam::scope;
    use std::time::Duration;

    let (snd, rcv) = unbounded();

    scope(|s| {
        s.spawn(|_| {
            for i in 0..5 {
                snd.send(i).unwrap();
                std::thread::sleep(Duration::from_millis(10));
            }
        });
    })
    .unwrap();

    let mut received = Vec::new();
    for _ in 0..5 {
        received.push(rcv.recv().unwrap());
    }

    assert_eq!(received, vec![0, 1, 2, 3, 4]);
}

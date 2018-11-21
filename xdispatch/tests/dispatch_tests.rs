// Based on <https://github.com/SSheldon/rust-dispatch/blob/master/src/lib.rs>
// (by Steven Sheldon, licensed under the MIT license)

extern crate xdispatch;

use std::sync::{Arc, Mutex};
use std::time::{Duration, Instant};
use xdispatch::*;

fn async_increment(queue: &Queue, num: &Arc<Mutex<i32>>) {
    let num = num.clone();
    queue.async(move || {
        let mut num = num.lock().unwrap();
        *num += 1;
    });
}

#[test]
fn test_serial_queue() {
    let q = Queue::create("", QueueAttribute::Serial);
    let mut num = 0;

    q.sync(|| num = 1);
    assert_eq!(num, 1);

    assert_eq!(q.sync(|| num), 1);
}

#[test]
fn test_sync_owned() {
    let q = Queue::create("", QueueAttribute::Serial);

    let s = "Hello, world!".to_string();
    let len = q.sync(move || s.len());
    assert_eq!(len, 13);
}

#[test]
fn test_serial_queue_async() {
    let q = Queue::create("", QueueAttribute::Serial);
    let num = Arc::new(Mutex::new(0));

    async_increment(&q, &num);

    // Sync an empty block to ensure the async one finishes
    q.sync(|| ());
    assert_eq!(*num.lock().unwrap(), 1);
}

#[test]
fn test_after() {
    let q = Queue::create("", QueueAttribute::Serial);
    let group = Group::create();
    let num = Arc::new(Mutex::new(0));

    let delay = Duration::from_millis(5);
    let num2 = num.clone();
    let guard = group.enter();
    let start = Instant::now();
    q.after(delay, move || {
        let mut num = num2.lock().unwrap();
        *num = 1;
        guard.leave();
    });

    // Wait for the previous block to complete
    assert!(group.wait_timeout(Duration::from_millis(5000)));
    assert!(start.elapsed() >= delay);
    assert_eq!(*num.lock().unwrap(), 1);
}

#[test]
fn test_queue_label() {
    let q = Queue::create("com.example.rust", QueueAttribute::Serial);
    assert_eq!(q.label(), "com.example.rust");
}

#[test]
fn test_apply() {
    let q = Queue::create("", QueueAttribute::Serial);
    let num = Arc::new(Mutex::new(0));

    q.apply(5, |_| *num.lock().unwrap() += 1);
    assert_eq!(*num.lock().unwrap(), 5);
}

#[test]
fn test_foreach() {
    let q = Queue::create("", QueueAttribute::Serial);
    let mut nums = [0, 1];

    q.foreach(&mut nums, |x| *x += 1);
    assert_eq!(nums, [1, 2]);
}

#[test]
fn test_map() {
    let q = Queue::create("", QueueAttribute::Serial);
    let nums = vec![0, 1];

    let result = q.map(nums, |x| x + 1);
    assert_eq!(result, [1, 2]);
}

#[test]
fn test_barrier_sync() {
    let q = Queue::create("", QueueAttribute::Concurrent);
    let num = Arc::new(Mutex::new(0));

    async_increment(&q, &num);
    async_increment(&q, &num);

    let num2 = num.clone();
    let result = q.barrier_sync(move || {
        let mut num = num2.lock().unwrap();
        if *num == 2 {
            *num = 10;
        }
        *num
    });
    assert_eq!(result, 10);

    async_increment(&q, &num);
    async_increment(&q, &num);

    q.barrier_sync(|| ());
    assert_eq!(*num.lock().unwrap(), 12);
}

#[test]
fn test_barrier_async() {
    let q = Queue::create("", QueueAttribute::Concurrent);
    let num = Arc::new(Mutex::new(0));

    async_increment(&q, &num);
    async_increment(&q, &num);

    let num2 = num.clone();
    q.barrier_async(move || {
        let mut num = num2.lock().unwrap();
        if *num == 2 {
            *num = 10;
        }
    });

    async_increment(&q, &num);
    async_increment(&q, &num);

    q.barrier_sync(|| ());
    assert_eq!(*num.lock().unwrap(), 12);
}

#[test]
fn test_suspend() {
    let q = Queue::create("", QueueAttribute::Serial);
    let num = Arc::new(Mutex::new(0));

    // Suspend the queue and then dispatch some work to it
    let guard = q.suspend();
    async_increment(&q, &num);

    // Sleep and ensure the work doesn't occur
    ::std::thread::sleep(Duration::from_millis(5));
    assert_eq!(*num.lock().unwrap(), 0);

    // But ensure the work does complete after we resume
    guard.resume();
    q.sync(|| ());
    assert_eq!(*num.lock().unwrap(), 1);
}

#[test]
fn test_group() {
    let group = Group::create();
    let q = Queue::create("", QueueAttribute::Serial);
    let num = Arc::new(Mutex::new(0));

    let num2 = num.clone();
    group.async(&q, move || {
        let mut num = num2.lock().unwrap();
        *num += 1;
    });

    let guard = group.enter();
    assert!(!group.is_empty());
    let num3 = num.clone();
    q.async(move || {
        let mut num = num3.lock().unwrap();
        *num += 1;
        guard.leave();
    });

    let num4 = num.clone();
    group.notify(&q, move || {
        let mut num = num4.lock().unwrap();
        if *num == 2 {
            *num = 10;
        }
    });

    group.wait();
    assert!(group.is_empty());

    // Our group is empty, but the notify may not have finished yet
    q.sync(|| ());
    assert_eq!(*num.lock().unwrap(), 10);
}

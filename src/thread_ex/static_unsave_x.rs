//! # Example
//!
//! https://doc.rust-lang.org/book/ch16-01-threads.html
//!
//!

#![allow(unused)]
use std::thread;
use std::time::Duration;

fn fn1() {
    static mut X: &str = "1";
    let handle = thread::spawn(move || {
        for i in 1..10 {
            unsafe {
                X = "2";
                println!("hi number i={}, x={} from 1 the spawned thread!", i, X);
            }
            thread::sleep(Duration::from_millis(1));
        }
    });
    thread::spawn(move || {
        for i in 1..10 {
            unsafe {
                // X = "2";
                println!("hi number i={}, x={} from 2 the spawned thread!", i, X);
            }
            thread::sleep(Duration::from_millis(100));
        }
    });
    unsafe { X = "3" };
    for i in 1..5 {
        unsafe {
            println!("hi number i={}, x={} from the main thread!", i, X);
        }
        thread::sleep(Duration::from_millis(1));
    }
    handle.join();
    // handle.join().unwrap();
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test1() {
        fn1();
    }
}

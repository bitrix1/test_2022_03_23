#![allow(unused)]
// use tokio::join;
use futures::future::join_all;
use tokio::time::{sleep, Duration}; //{join_all, ok, err};
                                    // std::future::join
                                    // tokio::join
                                    // core::future::join

async fn for_test(id: i32) {
    // let val: Vec<u8> = vec![0; 1024 * 1024 * 128];
    println!("id: {:?}", id);
    let mut last = 0;
    for x in 1..1000 {
        // use regex::Regex;
        // let re = Regex::new(r"^\d{4}-\d{2}-\d{2}$").unwrap();
        // re.is_match("2014-01-01");
        // sleep(Duration::from_micros(100000));
        // assert!(re.is_match("2014-01-01"));

        // if x > 12 {
        //     break;
        // }
        // last = x;
    }
    // println!("for_test end size val = {:?}mb", val.len() / 1024 / 1024);
}
pub async fn my_test() {
    let mut tasks = vec![];
    for i in 0..20 {
        tasks.push(for_test(i));
        // tasks.push(sleep(Duration::from_secs(15)));
        // tasks.push(sleep(Duration::from_micros(100)));
    }
    tokio::spawn(join_all(tasks)).await;
    // println!("Future test end.{:?}", Duration::from_millis(100));
    // println!("Size 1024 * 1024 * 1024 * 1000 / 8 = {:?}", 1024 * 1024 * 1024 * 100 / 8);
    assert!(true);
}

#[cfg(test)]
mod tests {
    use tokio::sync::futures;

    use super::*;

    #[tokio::test(flavor = "multi_thread", worker_threads = 20)]
    // #[tokio::test]
    async fn test1() {
        my_test().await;
    }

    #[test]
    fn test2() {
        #[tokio::main]
        async fn main() {
            println!("Hello world");
        }
    }
}

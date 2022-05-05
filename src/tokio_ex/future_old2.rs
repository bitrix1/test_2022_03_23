//https://play.rust-lang.org/?version=stable&mode=debug&edition=2021&gist=b800c3e786b01e21f68eee60b6b58132
#![allow(unused)]
// use std::sync::RwLock;

use futures::future::join_all; //iter.into_iter().map
use tokio::sync::{RwLock, RwLockReadGuard};
use tokio::task;
use tokio::time::{sleep, Duration, Sleep};

async fn for_test(i: i32) -> Sleep {
    // let val: Vec<u8> = vec![0; 1024 * 1024 * 128]; //128Mb
    if i == 1 {
        // return sleep(Duration::from_secs(3));
        std::thread::sleep(Duration::from_secs(3));
        println!("id: {:?}", i);
        return sleep(Duration::from_secs(0));
    }
    println!("id: {:?}", i);
    return sleep(Duration::from_secs(0));
}

pub async fn main() {
    let mut tasks = vec![];
    // let multi = std::sync::Arc::new(RwLock::new(0));
    let multi = std::sync::Arc::new(RwLock::new(0));
    // let ii = RwLock::new(vec![0; 10]);
    // let rw: 'static RwLock<i32> = RwLock::new(0);
    // let ii: &'static= RwLock::new(0);

    for _ in 0..1 {
        let join = task::spawn(async move {
            let guard = ii.read().await;
            // let guard = RwLockReadGuard::map(guard, |f| &f);
            // let i = *guard;
        });
        tasks.push(join);
        // The returned result indicates that the task failed.
        // assert!(join.await.is_err());
    }

    for val in tasks {
        val.await;
    }
    // tokio::spawn(join_all(tasks)).await;
}

#[cfg(test)]
mod tests {
    use super::*;
    use tokio::sync::futures;
    #[tokio::test(flavor = "multi_thread", worker_threads = 20)]
    // #[tokio::test(flavor = "multi_thread")]
    // #[tokio::test]
    async fn test1() {
        main().await;
    }

    #[test]
    fn test2() {
        #[tokio::main]
        async fn main() {
            println!("Hello world");
        }
    }
}

// for _ in 0..1 {
//     let join = task::spawn(async {
//         let guard = ii.read().await;
//         let guard = RwLockReadGuard::map(guard, |f| &f);
//         let i = *guard;
//         // let m = multi.read().await;
//         // drop(m);
//         let val: Vec<u8> = vec![0; 1024 * 1024 * 128]; //128Mb
//         if *i == 1 {
//             sleep(Duration::from_secs(10)).await;
//             println!("id: {:?}", i);
//         } else {
//             // sleep(Duration::from_secs(100)).await;
//             // let val1 = val.len();
//             for v in val {
//                 v.to_string();
//                 v + 1;
//                 // println!("id: {:?}", i);
//                 // println!("to_string: {:?}", v.to_string());
//             }
//             println!("id: {:?}", i);
//         }
//         // drop(i);
//         // drop(guard);
//         // panic!("something bad happened!")
//     });
//     tasks.push(join);
//     // The returned result indicates that the task failed.
//     // assert!(join.await.is_err());
// }

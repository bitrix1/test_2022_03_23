//https://play.rust-lang.org/?version=stable&mode=debug&edition=2021&gist=b800c3e786b01e21f68eee60b6b58132
// https://play.rust-lang.org/?version=stable&mode=debug&edition=2021&gist=389de485b8398df375e7fde4c81029c7
//The problem is that you move the only handle to the Arc in and then clone it. You want to clone it on each iteration of the loop and then move the clone into the thread.
// https://users.rust-lang.org/t/sharing-variable-among-threads-safely/15305
#![allow(unused)]
// use std::sync::RwLock;

use futures::future::join_all; //iter.into_iter().map
use std::sync::Arc;
use tokio::runtime;
use tokio::sync::{RwLock, RwLockReadGuard, RwLockWriteGuard};
use tokio::time::{sleep, Duration, Sleep};
use tokio::{spawn, task};
struct Test1 {
    count: i32,
}
pub async fn main() -> Result<(), ()> {
    // spawn(async {}).await;
    let mut tasks = vec![];
    let mut ii = Arc::new(RwLock::new(Test1 { count: 0 }));
    // let mut ii = Arc::new(RwLock::new(0));
    for iii in 0..2 {
        // let i = ii.clone();
        let i = Arc::clone(&ii);
        let join = spawn(async move {
            // println!("to_string: {:?}", i3);
            loop {
                let guard = i.read().await;
                let guard_r = RwLockReadGuard::map(guard, |f| &f.count);
                println!("address = {:p}", &guard_r);
                if *guard_r >= 100 {
                    return;
                }
                drop(guard_r);
                let guard = i.write().await;
                let mut guard_w = RwLockWriteGuard::map(guard, |f| f);
                guard_w.count += 1;

                drop(guard_w);
            }
        });
        tasks.push(join);
    }

    for val in tasks {
        val.await;
    }

    Ok(())
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

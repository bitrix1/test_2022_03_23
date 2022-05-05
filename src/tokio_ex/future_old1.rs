#![allow(unused)]
use futures::future::join_all; //iter.into_iter().map
use tokio::time::{sleep, Duration, Sleep};

async fn for_test(id: i32) -> Sleep {
    // let val: Vec<u8> = vec![0; 1024 * 1024 * 128]; //128Mb
    if id == 1 {
        // return sleep(Duration::from_secs(3));
        std::thread::sleep(Duration::from_secs(3));
        println!("id: {:?}", id);
        return sleep(Duration::from_secs(0));
    }
    println!("id: {:?}", id);
    return sleep(Duration::from_secs(0));
}
pub async fn main() {
    let mut tasks = vec![];
    for i in 0..5 {
        tasks.push(for_test(i));
        // tasks.push(sleep(Duration::from_secs(0)));
        // tasks.push(sleep(Duration::from_micros(100)));
    }
    tokio::spawn(join_all(tasks)).await;
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

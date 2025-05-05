use std::process::Output;
use std::thread;
use std::time::Duration;
use std::future::Future;

use trpl::Either;


fn slow(name: &str, ms: u64){
    thread::sleep(Duration::from_millis(ms));
    println!("'{name}' ran for {ms}ms");
}

async fn my_slow() -> String{
    trpl::sleep(Duration::from_millis(1000)).await;
    String::from("I finished")
}

async fn timeout<F: Future>(
    future_to_try: F,
    max_time: Duration
) -> Result<F::Output, Duration> {
    match trpl::race(future_to_try, trpl::sleep(max_time)).await {
        Either::Left(output) => Ok(output),
        Either::Right(_) => Err(max_time)
    }
}

fn main() {
    trpl::run(async{
        // let one_ms = Duration::from_millis(1);

        // let a = async {
        //     println!("'a' started.");
        //     //trpl::sleep(one_ms).await; //awit to let runtime switch to do another async task
        //     trpl::yield_now().await;
        //     slow("a", 30);
        //     // trpl::sleep(one_ms).await;
        //     trpl::yield_now().await;
        //     slow("a", 10);
        //     // trpl::sleep(one_ms).await;
        //     trpl::yield_now().await;
        //     slow("a", 20);
        //     // trpl::sleep(one_ms).await;
        //     trpl::yield_now().await;
        //     println!("'a' finished.");
        // };
        // let b = async {
        //     println!("'b' started.");
        //     //trpl::sleep(one_ms).await;
        //     trpl::yield_now().await;
        //     slow("b", 75);
        //     //trpl::sleep(one_ms).await;
        //     trpl::yield_now().await;
        //     slow("b", 10);
        //     //trpl::sleep(one_ms).await;
        //     trpl::yield_now().await;
        //     slow("b", 15);
        //     //trpl::sleep(one_ms).await;
        //     trpl::yield_now().await;
        //     slow("b", 350);
        //     //trpl::sleep(one_ms).await;
        //     trpl::yield_now().await;
        //     println!("'b' finished.");
        // };

        // trpl::race(a,b).await;
    
        match timeout(my_slow(), Duration::from_secs(2)).await {
            Ok(message) => println!("Succeeded with '{message}'"),
            Err(duration) => {
                println!("Failed after {} seconds", duration.as_secs())
            }
        }
    })
}
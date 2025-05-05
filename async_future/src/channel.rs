use std::time::Duration;
use std::pin::{pin, Pin};
use time::OffsetDateTime;
use std::future::Future;


fn main() {
    trpl::run(async {
        let (tx, mut rx) = trpl::channel::<String>();    

        let tx1 = tx.clone();
        let tx1_fut = pin!(async move { 
            // use move to let async take ownership from tx then the tx will drop after sending last word and 
            // rx.recv().await will produce None then rx loop will automatically end after receive the last word

            let vals = [
            String::from("more"),
            String::from("message"),
            String::from("for"),
            String::from("you")
            ];

            for val in vals {
                tx1.send(val).unwrap();
                trpl::sleep(Duration::from_millis(500)).await;
            }
        });

        let tx_fut = pin!(async move { 
            // use move to let async take ownership from tx then the tx will drop after sending last word and 
            // rx.recv().await will produce None then rx loop will automatically end after receive the last word

            let vals = [
            String::from("hi"),
            String::from("from"),
            String::from("the"),
            String::from("future")
            ];

            for val in vals {
                tx.send(val).unwrap();
                trpl::sleep(Duration::from_millis(500)).await;
            }
        });

        let rx_fut = pin!(async move{
            while let Some(val) = rx.recv().await {
                let now = OffsetDateTime::now_utc().second() as f64 + OffsetDateTime::now_utc().nanosecond() as f64 * 1e-9;
                println!("received '{val}', at time {now}");
            }
        });

        // trpl::join3(tx1_fut, rx_fut, tx_fut).await;
        // trpl::join!(tx_fut, rx_fut, tx1_fut);

        // let futures: Vec<Pin<Box<dyn Future<Output = ()>>>> = vec![Box::pin(tx1_fut), Box::pin(rx_fut), Box::pin(tx_fut)];
        let futures: Vec<Pin<&mut dyn Future<Output = ()>>> = vec![tx1_fut, rx_fut, tx_fut]; //use macros pin!

        trpl::join_all(futures).await;

        let a = async {1u32};
        let b = async {"Hello!"};
        let c = async {true};

        let (a_result, b_result, c_result) = trpl::join!(a,b,c);
        println!("{a_result}, {b_result}, {c_result}");

        let slow = async {
            println!("'slow' started.");
            trpl::sleep(Duration::from_millis(500)).await;
            println!("'slow' finished.");
        };

        let fast = async {
            println!("'fast' started.");
            trpl::sleep(Duration::from_millis(100)).await;
            println!("'fast' finished.");
        };

        trpl::race(slow, fast).await;
    });
}
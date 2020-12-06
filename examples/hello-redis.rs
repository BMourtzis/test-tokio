use mini_redis::client;
use tokio::sync::{mpsc, oneshot};
use test_tokio::utils::Command::{Set, Get};

#[tokio::main]
pub async fn main() {
    let (tx, mut rx) = mpsc::channel(32);

    //Redis Manager
    let mangager = tokio::spawn(async move {
        let mut client = client::connect("127.0.0.1:6379").await.unwrap();

        while let Some(cmd) = rx.recv().await {
            match cmd {
                Get { key, resp } => {
                    let res = client.get(&key).await;
                    //ignore errors
                    let _ = resp.send(res);
                }
                Set { key, val, resp} => {
                    let res = client.set(&key, val).await;
                    //ignore erros
                    let _ = resp.send(res);
                }
            }
        }
    });

    let tx2 = tx.clone();

    //Tokio Spawner
    let t1 = tokio::spawn(async move {
        let (resp_tx, resp_rx) = oneshot::channel();
        let cmd = Get {
            key: "hello".to_string(),
            resp: resp_tx
        };

        //send
        tx.send(cmd).await.unwrap();

        //Await repsonse
        let res = resp_rx.await;
        println!("Got = {:?}", res);
    });

    let t2 = tokio::spawn(async move {
        let (resp_tx, resp_rx) = oneshot::channel();
        let cmd = Set {
            key: "foo".to_string(),
            val: "bar".into(),
            resp: resp_tx
        };

        //send
        tx2.send(cmd).await.unwrap();

        //await response
        let res = resp_rx.await;
        println!("GOT = {:?}", res);
    });

    t1.await.unwrap();
    t2.await.unwrap();
    mangager.await.unwrap();
}
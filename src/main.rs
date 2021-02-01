use tokio::sync::oneshot;
use test_tokio::my_select::MySelect;

#[tokio::main]
async fn main() {
    let (tx1, rx1) = oneshot::channel();
    let (tx2, rx2) = oneshot::channel();

    MySelect{
        rx1, rx2
    }.await;
}
use super::Events;
use tokio::sync::mpsc::Receiver;
pub async fn start(mut input: Receiver<Events>) -> i32 {
    tokio::spawn(async move {
        loop {
            match input.recv().await {
                Some(Events::Log) => {
                    println!("Log");
                }
                Some(Events::Say) => {
                    println!("Hello");
                }
                None => {}
            }
        }
    });
    0
}

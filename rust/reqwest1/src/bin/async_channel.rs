use reqwest1::errors;
use std::time::Duration;
use tokio::time;

#[derive(Debug)]
struct Task {
    id: i32,
    sender_id: i32,
    receiver_id: i32,
}

impl Drop for Task {
    fn drop(&mut self) {
        /*
        if self.sender_id != 0 {
            return;
        }
        */
        println!(
            "{} -> {}: {} dropped",
            self.sender_id, self.receiver_id, self.id
        );
    }
}

#[tokio::main]
async fn main() -> errors::Result<()> {
    let mut handles = vec![];
    let (sender, receiver) = async_channel::bounded::<Task>(1);

    for i in 0..2 {
        let receiver = receiver.clone();
        let handle = tokio::spawn(async move {
            let receiver_id = i;
            loop {
                match receiver.recv().await {
                    Ok(mut task) => {
                        task.receiver_id = receiver_id;
                        //println!("receiver {}: {:?}", receiver_id, task);
                    }
                    Err(err) => {
                        println!("receiver {}: {:?}", receiver_id, err);
                    }
                }
                time::sleep(Duration::from_millis(1000)).await;
            }
        });
        handles.push(handle);
    }

    //receiver.close();

    for i in 0..5 {
        let sender = sender.clone();
        let handle = tokio::spawn(async move {
            let mut id = 0;
            let sender_id = i;
            let receiver_id = -1;
            loop {
                let task = Task {
                    id,
                    sender_id,
                    receiver_id,
                };
                id = id + 1;
                match sender.send(task).await {
                    Ok(()) => {
                        //println!("sender {}: send task {} successful", sender_id, id);
                    }
                    Err(err) => {
                        println!("sender {}: send task {} failed {:?}", sender_id, id, err);
                    }
                }
            }
        });
        handles.push(handle);
    }

    //sender.close();

    for handle in handles {
        handle.await?;
    }

    Ok(())
}

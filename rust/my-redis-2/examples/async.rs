async fn say_world() {
    println!("world");
}

// #[tokio::main]
// async fn main() {
//     // Calling `say_world()` does not execute the body of `say_world()`.
//     let op = say_world();
//     // let n: i32 = op;

//     // This println! comes first
//     println!("hello");

//     // Calling `.await` on `op` starts executing `say_world`.
//     op.await;
// }

async fn task1() {
    // Calling `say_world()` does not execute the body of `say_world()`.
    let op = say_world();
    // let n: i32 = op;

    // This println! comes first
    println!("hello");

    // Calling `.await` on `op` starts executing `say_world`.
    op.await;
}

use tokio::task;

async fn task2() {
    task::spawn(async {
        // ...
        println!("spawned task done!")
    });

    // Yield, allowing the newly-spawned task to execute first.
    // task::yield_now().await;
    println!("main task done!");
}

fn main() {
    let mut rt = tokio::runtime::Runtime::new().unwrap();
    rt.block_on(task2())
}

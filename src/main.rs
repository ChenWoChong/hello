use tokio::sync::{mpsc, oneshot};
use tokio::time;

#[tokio::main]
async fn main() {
    let ta = tokio::task::spawn(async {
        println!("task a");
        time::sleep(std::time::Duration::from_secs(3)).await;
        1
    });

    let tb = tokio::task::spawn(async {
        println!("task b");
        2
    });
    let tc = tokio::task::spawn(async {
        println!("task c");
        3
    });

    // let mut tasks = Vec::with_capacity(3);
    // tasks.push(ta);
    // tasks.push(tb);
    // tasks.push(tc);

    // let mut outputs = Vec::with_capacity(tasks.len());
    // for (i, task) in tasks.into_iter().enumerate() {
    //     println!("iter tasks {i}");
    //     outputs.push(task.await.unwrap());
    // }

    // println!("{:?}\n---------------\n", outputs);

    // let (r1,r2, r3) = tokio::join!(ta, tb, tc);
    // println!("{} {} {}",r1.unwrap(), r2.unwrap(), r3.unwrap());

    let ret = tokio::select! {
        r = ta=> r.unwrap(),
        r = tb=> r.unwrap(),
        r = tc=> r.unwrap(),
    };

    println!("got ret is {ret}");
    println!("--------------");

    let mut db = vec![1, 2, 3, 4, 5, 6, 7, 8];

    // let (tx, mut rx) = mpsc::channel::<i32>(100);
    let (tx, mut rx) = mpsc::channel::<(i32, oneshot::Sender<bool>)>(100);

    let tx_a = tx.clone();
    let tx_b = tx.clone();

    let task_a = tokio::task::spawn(async move {
        println!("in task_a 1");
        time::sleep(std::time::Duration::from_secs(1)).await;
        println!("in task_a 2");
        let (rsp_tx, rsp_rx) = oneshot::channel::<bool>();
        if let Err(_) = tx_a.send((100, rsp_tx)).await {
            println!("end task_a");
        }

        if let Ok(ret) = rsp_rx.await {
            if ret {
                println!("task_a fish with success");
            } else {
                println!("task_a fish with failure");
            }
        } else {
            println!("oneshot send dropped");
        }
    });

    let task_b = tokio::task::spawn(async move {
        println!("in task_b");
        let (rsp_tx, rsp_rx) = oneshot::channel::<bool>();
        if let Err(_) = tx_b.send((100, rsp_tx)).await {
            println!("end task_b");
        }

        if let Ok(ret) = rsp_rx.await {
            if ret {
                println!("task_b finish with success");
            } else {
                println!("task_b finish with failure");
            }
        } else {
            println!("oneshot send dropped");
        }
    });

    let task_c = tokio::task::spawn(async move {
        while let Some((n, rsp_tx)) = rx.recv().await {
            println!("got n = {n}");
            db[5] = n;
            println!("{:?}", db);
            rsp_tx.send(true).unwrap();
        }
    });

    _ = task_a.await;
    _ = task_b.await;
    _ = task_c.await;
}

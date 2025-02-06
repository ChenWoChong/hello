use std::time::Duration;
use std::time::SystemTime;
use tokio::{
    fs::File,
    io::{AsyncReadExt, AsyncWriteExt},
    task, time,
};

#[tokio::main(flavor = "current_thread")]
async fn main() {
    // tokio::runtime::Builder::new_current_thread()
    //     .enable_all()
    //     .build()
    //     .unwrap()
    //     .block_on(foo())
    foo().await;
    let _ = write().await;
    let _ = read().await;

    let mut interval = time::interval(Duration::from_millis(10));
    println!("now: {:?}", SystemTime::now());
    interval.tick().await;
    println!("now: {:?}", SystemTime::now());
    interval.tick().await;
    println!("now: {:?}", SystemTime::now());
    interval.tick().await;

    let ts_a = task::spawn(async move {
        let a = "hahah";
        println!("task print: {}", a);
        a
    });

    let res = ts_a.await.unwrap();
    assert_eq!(res, "hahah");

    let ts_b = task::spawn(async move {
        panic!("some bad thing happend!");
    });

    assert_eq!(ts_b.await.is_err(), true);

    let ops = vec![1, 2, 3];
    let mut tasks = Vec::with_capacity(ops.len());
    for num in ops {
        tasks.push(task::spawn(do_some_idx(num)));
    }

    let mut outputs = Vec::with_capacity(tasks.len());
    for t in tasks {
        outputs.push(t.await.unwrap());
    }
    println!("{:?}", outputs);
}

pub async fn foo() {
    let a = async {
        println!("hello foo");
    };
    a.await;
    async move {}.await;
}

async fn write() -> std::io::Result<()> {
    let mut f = File::create("test.txt").await.unwrap();
    f.write_all(b"hello world, wochong").await.unwrap();
    Ok(())
}

async fn read() -> std::io::Result<()> {
    let mut f = File::open("test.txt").await.unwrap();
    let mut cnt = vec![];
    f.read_to_end(&mut cnt).await.unwrap();
    println!("read content: {} {:?}", cnt.len(), cnt);
    Ok(())
}

async fn do_some_idx(id: i32) -> String {
    let f = format!("this is china {}", id);
    println!("{}", f);
    f
}

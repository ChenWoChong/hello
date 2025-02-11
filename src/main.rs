use std::sync::{atomic::AtomicU32, Arc};
use tokio::sync::{Mutex, RwLock};

#[tokio::main]
async fn main() {
    let db = vec![1, 2, 3, 4, 5, 6, 7, 8, 9];
    println!("{:?}", db);
    let arc_db = Arc::new(Mutex::new(db));
    let arc_db2 = arc_db.clone();
    let arc_db3 = arc_db.clone();

    let task_a = tokio::task::spawn(async move {
        let mut db = arc_db.lock().await;
        db[3] = 100;
        assert_eq!(db[3], 100);
        println!("{:?}", db);
    });

    let task_b = tokio::task::spawn(async move {
        let mut db = arc_db2.lock().await;
        db[3] = 111;
        assert_eq!(db[3], 111);
        println!("{:?}", db);
    });

    let _ = task_a.await.unwrap();
    _ = task_b.await.unwrap();
    println!("{:?}", arc_db3);

    let db = RwLock::new(6);
    {
        let a = db.read().await;
        let b = db.read().await;
        assert_eq!(*a, 6);
        assert_eq!(*b, 6);
    }

    {
        let mut r = db.write().await;
        *r = 8;
        // let c = db.read().await;
        // assert_eq!(*c, 8);
        // 不能读取，因为写锁没有释放，会死锁等待
    }
    {
        let c = db.read().await;
        assert_eq!(*c, 8);
    }

    let _atom_num = AtomicU32::new(5);
    let _arc_atomic_num = Arc::new(AtomicU32::new(5));

    let mut an = AtomicU32::new(6);
    *an.get_mut() = 12;
    assert_eq!(*an.get_mut(), 12);
}

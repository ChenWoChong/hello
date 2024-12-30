fn main() {
    let data: Vec<i32> = vec![1, 2, 3, 4];
    let data1 = &data;
    println!(
        "addr of value:\t{:p} ({:p}),\naddr of  data:\t{:p},\naddr of data1:\t{:p},\naddr of   vec:\t{:p} ",
        &data, data1, &&data, &data1, data.as_ptr()
    );

    // 打印 data 的地址（存储在栈上）
    println!("Address of data on stack:\t{:p}", &data);

    // 打印 Vec 的堆内存地址（数据存储在堆上）
    let ptr = data.as_ptr();
    println!("Address of Vec's heap memory:\t{:?}", ptr);

    // 打印 Vec 中每个元素的地址
    for (i, element) in data.iter().enumerate() {
        println!("Address of element {}: {:p}", i, element);
    }
}

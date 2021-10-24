use std::io;

fn main() {
    let a = [1,2,3,4,5];

    println!("please enter en array index");

    let mut index = String::new();

    io::stdin()
        .read_line(&mut index)
        .expect("failed to read line");
    let index: usize = index.trim().parse().expect("index entered was not a number");

    let element = a[index];
    // 一旦输入的index越界就会报运行时错
    println!("the value of the element at index {} is: {}", index, element); // 超出5 会 panic，数组越界
}
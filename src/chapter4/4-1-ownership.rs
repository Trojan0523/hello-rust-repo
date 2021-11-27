// 管理内存方式
// 1. 亲自分配和释放内存
// 2. GC 程序运行时不断寻找不再使用的内存
// 3. Rust: 通过所有权系统管理内存 没有 GC

// 三点规则
// Rust 每个值都有一个被其成为所有者的变量
// 值在任何时刻有且只有一个所有者
// 当所有者(变量)离开作用域时，值被丢弃 没有闭包？

// 变量作用域

// let s = "hello";

// {
//     // s在此之前无效，未声明
//     let s = "hello";
//     // use s
// }
// s is invalid
// 使用from函数基于字符串字面量创建string， 可以存储编译时未知大小的文本
// let s = String::from("hello");
// :: 是运算符，允许特定的from函数置于String类型的命名空间(namespace)
// 修改此类字符串
fn main () {
    let mut s = String::from("hello");

    s.push_str(", world");
    
    println!("{}", s);
    {
        let x = 6;
        let y = x;
        println!("资源池尾 }} 释放资源{}", y)
    }
    swap_variable();
    clone_variable();
    ownership_and_function();
    return_value_and_scope();
    transfer_return_value_ownership();
}

// 内存与分配

// Rust 内存在拥有他的变量离开作用域后就被自动释放

// 离开作用域时，Rust会调用一个特殊的函数，drop(), string的设置者可以放置释放内存的代码，} 处自动调用drop
// 在C++中，变量在生命周期结束时释放资源的模式有时候叫资源获取即初始化(Resource Acquisition is Initialization) RAII

fn swap_variable() {
    let s1 = String::from("hello");
    // let s2 = s1;
    println!("{}", s1);
}
// string 底层 有一个指针(内存地址) 一个长度变量，一个容量变量
// 将 值"s1" 绑定给s1的string内存中：拷贝了指针，长度和容量，不过没有复制指针指向的堆上数据
// 当 s1 和 s2 离开作用域，他们会尝试释放相同的内存，这个叫二次释放错误，也是内存安全的bug之一，两次释放内存会导致内存污染，会存在潜在的安全漏洞
// s2被创建后使用s1会报错，Rust禁用使用无效的引用

// 这里的复制不是浅拷贝 深拷贝，是一个移动(move)的操作, 将s1移动到s2中

// Rust永远不会自动创建数据的深拷贝，任何自动的复制都可以被认为对运行时性能影响较小

// 变量与数据交换方式2 ： 克隆
fn clone_variable() {
    let s1 = String::from("hello");
    let s2 = s1.clone();
    println!("s1 = {}, s2 = {}", s1, s2)
    // 只在栈上的数据： 拷贝
    // copy类型： 整型 u32 布尔 bool 浮点数 f64 字符类型char 元组 (i32, i32)
}
fn ownership_and_function() {
    let s = String::from("hello"); // s 进入作用域
    takes_ownership(s); // s值移到作用域中
    let x = 5; // x进入作用域
    makes_copy(x); // x移动函数中，i32是 copy的，后面有可能可以继续使用s
    // println!("{}, {}", s, x); 报错，因为所有权已经转移到函数中并且函数退出时已经释放
} // 执行drop() 移除x, 然后是s， 因为s值已经被移走，所以不会有特殊操作

fn takes_ownership(some_string: String) { // some_string 进入作用域
    println!("{}", some_string);
} // some_string 移出做哟用于并调用drop方法，占用内存释放

fn makes_copy(some_integer: i32) { // some_integer 进入作用域
    println!("{}", some_integer);  
} // some_integer 移出作用域，不会有特殊操作


// return value and scope
// 返回值也能转移所有权
fn return_value_and_scope() {
    let s1 = gives_ownership(); // gives_ownership将返回值移给s1
    let s2 = String::from("hello"); // s2 进入作用域
    let s3 = takes_and_gives_back(s2); // s2被移动到 takes_and_gives_back中，也将返回值移给s3
    println!("{}, {}", s1, s3);
} // 执行drop， s3移除作用域丢弃，s2也是，不过早就到takes_and_gives_back作用域里面了，啥都不发生
// s1 也一样移除作用域并丢弃

fn gives_ownership() -> String {
    let some_string = String::from("hello");

    some_string // 返回some_string 并移除给调用的函数，此时所有权在调用方上
}

fn takes_and_gives_back (a_string: String) -> String { // a_string 进入作用域
    a_string // 返回a_string 并移出给调用函数
}

// 变量所有权遵循所有模式：将值赋给另一个变量时移动他，当持有对重数据值的变量离开作用域时，其值通过drop被清理掉，除非数据被移动到另一个变量所有

// 转移返回值的所有权 使用元组返回值的所有权

fn transfer_return_value_ownership() {
    let s1 = String::from("hello");

    let (s2, len) = calculate_length(s1);

    println!("the length of {} is {}", s2 ,len);
}
fn calculate_length(a_string: String) -> (String,usize) {
    let length = a_string.len(); // len() 返回字符串的长度

    (a_string, length)
}

// 这种写法比较麻烦，不过是为了理解所有权才这样写的，下一篇有引用和借用两个类型
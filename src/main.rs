// use ferris_says::say; // from the previous step
// use std::io::{stdout, BufWriter};

// fn main() {
//     let stdout = stdout();
//     let message = String::from("Hello fellow Buxiongyu!");

//     let width = message.chars().count();

//     let mut writer = BufWriter::new(stdout.lock());
//     say(message.as_bytes(), width, &mut writer).unwrap();
// }
/**
 * 输出输入的数字
 */
// use std::io;
// use rand::Rng;

// fn main () {
//     println!("Guess the number!"); // 输出
//     println!("Please input your guess.");

//     let secret_number = rand::thread_rng().gen_range(1..101);

//     println!("your secret_number is {}", secret_number);
//     // let x = 5;
//     // let y = 6;
//     // println!("x={}, y={}", x, y);
//     let mut guess = String::new(); // let创建一个变量， 同时mut设置了这个变量为不可变的数据流
//     // 后面的两个:: 是new String 类型的关联函数， 关联函数针对类型实现。

//     io::stdin().read_line(&mut guess) // &参数引用数据类型
//         .expect("fail to read line"); // 跟类型断言相似(jest等等测试框架，同样也有判断类型异常的提示信息)
//     io::stdin()
//         .read_line(&mut guess)
//         .expect("fail to read line");
//     println!("You guessed: {}", guess); // {} 为占位符打印值 

// }

/**
 * 比较猜测的数字和秘密数字
 */

 use std::io;
 use std::cmp::Ordering;
 use rand::Rng;

 fn main () {
    //  ---snip---
    // let result = 1.cmp(&2);
    // assert_eq!(Ordering::Less, result);

    // 用户输入的字符串 比对神秘数字用

    
    loop {
        let mut guess = String::new();

        let secret_number = rand::thread_rng().gen_range(1..101);

        println!("guessed password {}", secret_number);

        // 用一个io读取当前行 ，放入一个不可变的变量，达不到用户期待就log： Failed to read line
        io::stdin().read_line(&mut guess).expect("Failed to read line");

        let guess: u32 = match guess.trim().parse() { // match 用来匹配从错误崩溃转换到真正处理错误的惯用方法
            Ok(num) => num, // 成员枚举 Ok, Err ，ok对应的是新创建的guess变量
            Err(_) => continue,
        }; 
        // Rust允许用一个新值隐藏(shadow) guess 之前的值
        println!("You guessed {}", guess);
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too Small"),
            Ordering::Greater => println!("Too Big!"),
            Ordering::Equal => {
                println!("Bingo! you win!");
                break;
            },
        }
    }
    // let secret_number = rand::thread_rng().gen_range(1..100);
    
 }

// 学习总结：
//   外部引用crate 代码第三方库，跟npm类似，用 toml管理引入dependency
//   let 变量 mut创建不可变类型变量
//   match 匹配类型并且做处理 ，两个成员枚举 Ok Err，比较类似于 switch
//   use::std.io; 引入包函数
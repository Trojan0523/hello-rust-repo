fn main() {
    let number = 7;
    // if number <  5 {
    //     println!("condition is true");
    // } else {
    //     println!("condition is false");
    // }
    // if number { // rust 期望一个布尔却地道了一个整型，因为他没有强制类型转换，所以不支持隐式转换将整型转换为布尔
    if number != 0 {
        println!("the number is {}", number);
    }
    else_if_condition();
    let_use_if();
    nested_loop();
    loop_for_exec();
    loop_return();
    while_loop();
    for_loop();
    reverse_range();
}
// else if 处理多重条件

fn else_if_condition() {
    let number = 9;
    if number % 4 == 0 {
        println!("number is divisible by 4");
    } else if number % 3 == 0 {
        println!("number is divisible by 3");
    } else if number % 2 == 0 {
        println!("number is divisible by 2");
    } else {
        println!("number is not divisible by 2 3 4");
    }
}

// 在 let语句中使用if
fn let_use_if() {
    let condition = true;
    let number = if condition {
        5
    } else {
        6
    //    "six"  //6
    }; // 得保持类型一致，不然会编译错误，类型不确定 incompatible types
    println!("the number is {}", number);
}

// 使用循环重复执行
fn loop_for_exec() {
    let mut count = 10;
    loop {
        if count < 5 {
            count -= 1;
            println!("again");
        } else {
            break;
        }
    }
}

// 循环嵌套 
fn nested_loop() {
    let mut count = 0;
    'counting_loop: loop {
        println!("count = {}", count);
        let mut remaining = 10;
        loop {
            println!("remaining = {}", remaining);
            if remaining == 9 {
                break;
            }
            if count == 2 {
                break 'counting_loop;
            }
            remaining -= 1;
        }
        count += 1;
    }
    println!("End count = {}", count);
}

// 从循环返回
fn loop_return() {
    let mut counter = 0;
    let result = loop {
        counter += 1;
        if counter == 10 {
            break counter * 2;
        }
    };
    println!("the result is {}", result);
}

// while 条件循环

fn while_loop() {
    let mut number = 3;
    while number != 0 {
        println!("{}!", number);

        number = number - 1; 
    }
    println!("LEFT OFF!!!");
}

// for 循环遍历
fn for_loop() {
    // while 进行演示
    // let a = [10, 20, 30, 40, 50];
    // let mut index = 0;
    // while index < 5 {
    //     println!("the value is {}", a[index]);

    //     index += 1;
    // }
    // while遍历如果索引长度和测试条件不正确就会引起程序panic， 编译器增加运行时代码对每次循环进行条件检查
    let a = [10, 20, 30, 40, 50];
    for element in a.iter() {
        println!("the value is {}", element);
    }
}

//  用rev翻转range
fn reverse_range() {
    for number in (1..4).rev() {
        println!("{}!", number);
    }
    println!("LEFT OFF");
}

// summary: 变量，标量，复合数据类型，函数，注释， if表达式，循环 完成

// 尝试构建：
// 1. 互相转换摄氏和华氏度
// 2. 生成n阶斐波那契额数列
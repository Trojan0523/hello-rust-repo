// 声明新函数fn关键字，最好使用snake case规格风范


fn main() {
    println!("Hello world");
    another_function(5);
    print_labeled_measurement(61, 'h');
}

// 函数形参传入
fn another_function(x: i32) {
    println!("the value of x is {}", x);
}
// 设置的char是单引号 一个字母字符 
fn print_labeled_measurement(value: i32, unit_label: char) {
    // let y = 6;
    // 错误实例，由于其是一门基于表达式(expression-base)的语言
    // let x = (let y = 8); // 即也不能 x = y = 6;
    let x = 5;
    let y = {
        let x = 3;
        x + 1
    }; // 基于表达式的语言，最后返回的是x+1的表达式的值
    println!("the measurement is: {}{}{}{}", value, unit_label, x, y);
    // let five = five();
    println!("the value of function five is {}", five(1));
}
// 创建一个有返回值的函数
fn five(x: i32) -> i32 {
    x + 5 // 有效函数，同时返回值也设定了返回的类型 -> i32 ，不能在这里加分号，加分号提前断言
    // 报 expect `i32`, found `()` mismatched types
    // - help: consider removing this semicolon 考虑删除分号的hint
}


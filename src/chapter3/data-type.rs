// Rust 是静态类型语言，编译时就必须全部知道所有的类型
fn main() {
    // 标量(scalar) 类型代表单独的值，标量类型有：整型 浮点型 布尔类型 字符类型
    // let guess: u32 = "42".parse().expect("Not a number"); // 无符号整数unsigned int u8 16 32 64 128 arch - usize
    let guess_number: u32 = 256;
    println!("{}", guess_number);
    // 浮点型 f64 双精度 f32 单精度 浮点数
    let _x = 2.0; // f64
    let _y: f32 = 3.0; // f32
    // 加法
    let _sum = 5 + 10;
    // 减法
    let _difference = 95.5 - 4.3;
    // 除法
    let _quotient = 56.7 / 32.2;
    let _floored = 2 / 3; // result is 0
    // 取余
    let _remainder = 43 % 5;
    // 布尔类型，跟ts类似
    let _t = true;

    let _f: bool = false; // 显式指定类型注解
    // rust char 字符类型
    let _c = 'z';
    // let z = 'Z';
    // 复合类型 (compound type) 元组(tuple) 数组(array)
    // 使用包含在圆括号中的逗号分隔符的值列表创建一个元组
    let _tup: (i32, f64, u8) = (500, 6.4, 3);

    let tuple = (500, 6.4, 1);

    let (_b, a ,_c)  = tuple;

    println!("the value of y is {}", a);

    // 数组必须是每个元素类型相同，其长度是固定的，声明后，不能增加长度或者缩小长度
    let array = [1,2,3,4,5];

    let months = ["January", "February", "March", "April", "May", "June", "July",
              "August", "September", "October", "November", "December"];

    // 数组类型也可以定义，方括号第一个参数包含每个元素的类型，跟分号再跟元素数量
    let array1: [i32; 5] = [1,2,3,4,5];
    let array2 = [3; 5]; // 等价于 => let array2 = [3,3,3,3,3];
    // 通过数组下表的形式访问元素，同样从0开始
    // array2[0] // 3
    
}
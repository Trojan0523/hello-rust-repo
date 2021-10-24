// constant常量 和 mut 可变性
fn main () {
    let mut x = 5;
    println!("you set x is {}", x);
//  let x = 5; 还是变量
    x = 6;
    println!("you set x is {}", x); // 这里的声明值如果不设置 mut就不会改变 mutable constant 变量
    // const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3; // 这个才是常量
    
    // shadowing 作用域隐藏
    {
        // 这里新开了一个局部作用域，不会影响作用域外部的数值
        let x = x * 2;
        println!("the value of x in the inner scope is: {}", x);
    }
    println!("the value of x is: {}", x);

    let spaces = "   ";
    let spaces = spaces.len(); // 这里只是复用了他的名字，并且可以改变他的变量类型
    println!("spaces length is: {}", spaces);
    // let mut spaces = "   "; 
    // spaces = spaces.len(); 设置了mut之后不能更改他的变量类型

}
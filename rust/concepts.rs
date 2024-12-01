fn main() {
    //variable immutable
    let x = 5;

    //variable mutable
    let mut x = 5;

    //constants
    const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;

    //Shadowing
    let x = 5;
    let x = x + 1;
    {
        let x = x * 2;
        println!("The value of x in the inner scope is: {x}");
    }
    println!("The value of x is: {x}");

    //mut 与隐藏的另一个区别是，当再次使用 let 时，实际上创建了一个新变量，我们可以改变值的类型，并且复用这个名字。
    //let spaces = "   ";
    //let spaces = spaces.len();
    //不能改变变量的类型
    //let mut spaces = "   ";
    //spaces = spaces.len();

    //数据类型
    //整型、浮点型、布尔类型和字符类型（four bytes）。
    //复合类型：元组（tuple）和数组（array）
    //let variable : type = expersion;
    //let variable : [type : count] = expersion | [value : count];

    //函数
    //fn function_name(type:param, ...) -> type {
    //
    //}

    //语句和表达式
    //statements
    // xxx;
    // 语句不返回值。因此，不能把 let 语句赋值给另一个变量: let x = (let y = 6);
    //Expressions
    // 表达式会计算出一个值，并且你将编写的大部分 Rust 代码是由表达式组成的。
    // 5、5+6
    // { let x = 3; x + 1} 注意没有最后一句没有分号
    // 函数调用是一个表达式。宏调用是一个表达式。
    // 用大括号创建的一个新的块作用域也是一个表达式。
    // 在表达式的结尾加上分号，它就变成了语句，而语句不会返回值。

    //控制流
    //if 表达式 {} else if 表达式 {} ... else {}
    //注意：不像 Ruby 或 JavaScript 这样的语言，Rust 并不会尝试自动地将非布尔值转换为布尔值。
    //必须总是显式地使用布尔值作为 if 的条件。
    //语法糖：let number = if condition { 5 } else { 6 }; 这里 {} 中没有分号是表达式
    //循环 loop、while 和 for 
    //1.loop可以有返回值
    //2.循环标签：在多个循环之间消除歧义
    let mut count = 0;
    'counting_up: loop {
        println!("count = {count}");
        let mut remaining = 10;

        loop {
            println!("remaining = {remaining}");
            if remaining == 9 {
                break;
            }
            if count == 2 {
                break 'counting_up;
            }
            remaining -= 1;
        }

        count += 1;
    }
    println!("End count = {count}");
    // while 表达式 {  }
    // for item in containers {}
}

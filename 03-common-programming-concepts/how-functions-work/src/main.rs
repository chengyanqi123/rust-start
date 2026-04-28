// cn-docs @see https://kaisery.github.io/trpl-zh-cn/ch03-03-how-functions-work.html

// Rust 不关心函数定义所在的位置，只要函数被调用时出现在调用之处可见的作用域内就行。
// Rust 是一门基于表达式的语言，函数体由一系列语句组成，并且可以选择以一个表达式结束。
// - 语句（Statements）是执行一些操作但不返回值的指令。
// - 表达式（Expressions）计算并产生一个值。
//      - 函数调用是一个表达式。
//      - 宏调用是一个表达式。
//      - 用大括号创建的一个新的块作用域也是一个表达式。
fn main() {
    println!("Hello, Rust funtion!");

    // Rust 中并不能像这样进行连续赋值。因为不同于其他的一些语言，Rust 中赋值语句是没有返回值的。
    // let x = (let y = 6); // ❌

    // 可以将表达式的返回值进行变量的赋值
    let y = {
        let x = 3;
        x + 1
    };
    println!("The value of y is: {y}");

    another_function();
    print_labeled_measurement(5, 'h');

    println!("The value of x is: {}", five());
    println!("plus_one value of x is: {}", plus_one(-1));
}

// 简单函数示例
fn another_function() {
    println!("Another function.");
}

// 带参函数
fn print_labeled_measurement(value: i32, unit_label: char) {
    println!("The measurement is: {value}{unit_label}");
}

// 函数返回值
// 在 Rust 中，函数的返回值等同于函数体中最后一个表达式的值。
// 你也可以使用 return 关键字并指定一个值，从函数中提前返回；
fn five() -> i32 {
    5
}
fn plus_one(x: i32) -> i32 {
    x + 1
}
// 但如果在包含 `x + 1` 的行尾加上一个分号，把它从表达式变成语句，将会出现一个错误
// fn plus_one(x: i32) -> i32 {
//     x + 1;
// }
// error[E0308]: mismatched types

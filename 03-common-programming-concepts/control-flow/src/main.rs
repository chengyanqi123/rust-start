// cn-docs @see: https://kaisery.github.io/trpl-zh-cn/ch03-05-control-flow.html

fn main() {
    // if 表达式
    println!("============= statement_if start =============");
    statement_if();

    // loop 循环
    println!("============= statement_loop start =============");
    statement_loop();

    // while 循环
    println!("============= statement_while start =============");
    statement_while();

    // for 循环
    println!("============= statement_for start =============");
    statement_for();
}

fn statement_for() {
    // 循环遍历集合中的每一个元素
    let a = [10, 20, 30, 40, 50];
    for element in a {
        println!("the value is: {element}");
    }

    // rev() 用于反转 range
    for number in (1..4).rev() {
        println!("{number}!");
    }
    println!("LIFTOFF!!!");
}

fn statement_while() {
    let mut number = 3;
    while number != 0 {
        println!("{number}!");

        number -= 1;
    }
    println!("LIFTOFF!!!");
}

fn statement_loop() {
    // loop 语句将会陷入死循环，除非使用 `break` 关键字主动跳出循环
    // `break` 表达式后面加上想要返回的值；这个值会作为循环的返回值返回出来
    // 循环标签：
    //      如果循环中又套了循环，那么 break 和 continue 默认只作用于当前最内层的那个循环。
    //      可以循环加上一个 `循环标签（loop label）`
    //      然后把这个标签和 break 或 continue 一起使用
    //      这些关键字就会作用于被标记的循环，而不是最内层循环
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
                // 这样将会退出 `'counting_up` 标签的循环
                break 'counting_up;
            }
            remaining -= 1;
        }

        count += 1;
    }
    println!("End count = {count}");
}

fn statement_if() {
    let number = 8;
    // 只能只用 `bool` 类型的值作为 condition
    // ❌
    // if number {
    //     println!("number was three");
    // }
    // ✅
    if number != 0 {
        println!("number was three");
    }

    //
    if number % 4 == 0 {
        println!("number is divisible by 4");
    } else if number % 3 == 0 {
        println!("number is divisible by 3");
    } else if number % 2 == 0 {
        println!("number is divisible by 2");
    } else {
        println!("number is not divisible by 4, 3, or 2");
    }

    // 也可以将 if 表达式的返回值赋给一个变量
    let is_positive = if number > 0 { true } else { false };
    println!("is_positive value is: {is_positive}");
}

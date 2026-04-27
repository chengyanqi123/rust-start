// rand是一个外部库
// 需要执行 `cargo add rand` 进行安装
// 或者在 `Cargo.toml` 中手动添加 `rand` 的依赖
use rand::RngExt;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("========= Guess the number =========");

    // 随机生成一个 [1, 100] 范围内的整数
    // `rand::rng()`: 创建一个随机数生成器
    //  - `rand` 是 Cargo.toml 中引入的第三方 crate
    //  - `rng` 函数会返回一个当前线程可用的随机数生成器
    // `.random_range(1..=100)`: 从指定范围中随机取出一个值
    //  - `1..=100` 是一个包含结尾的范围表达式，表示从 1 到 100，包括 1 和 100
    //  - 如果写成 `1..100`，则是不包含结尾的范围，表示从 1 到 99
    //  - `random_range` 方法来自 `rand::RngExt` trait，所以文件顶部需要写 `use rand::RngExt;`
    // `let secret_number = ...`: 将生成出来的随机数绑定到变量 `secret_number`
    let secret_number = rand::rng().random_range(1..=100);
    // println!("The secret number is: {secret_number}");

    println!("Please input your guess.");

    loop {
        // let guess: 申明一个变量。在 Rust 中，变量默认是不可变的
        // let mut guess: 变量名前使用 `mut` 使一个变量可变
        // String::new(): 将创建一个字符实例
        let mut guess = String::new();

        // 如果没有使用 `use std::io`
        // 也可以写成 `std::io::stdin` 来使用该函数
        // `stdin` 函数返回一个 `std::io::Stdin` 的实例
        io::stdin()
            // read_line: 无论用户在标准输入中键入什么内容，都将其追加（不会覆盖其原有内容）到一个字符串中
            //  - 返回一个类型为 `Result` 的值，是一种枚举类型
            //      - Result 的成员是 Ok 和 Err
            //          - Ok 成员表示操作成功，内部包含成功时产生的值。
            //          - Err 成员则意味着操作失败，并且 Err 中包含有关操作失败的原因或方式的信息。
            //      - Result 的实例拥有 expect 方法
            // 将 `&mut guess`参数传递给 `read_line` 函数，让其将用户输入储存到这个字符串中
            // `&` 表示这个参数是一个 引用（reference），它允许多处代码访问同一处数据，而无需在内存中多次拷贝
            .read_line(&mut guess)
            // 使用 `expect` 处理可能出现的错误
            .expect("Failed to read line");

        // 将用户输入的字符串转换成数字
        // 前面的 `guess` 是一个 `String` 类型，里面保存的是用户从键盘输入的内容
        // `let guess: u32`: 重新声明一个同名变量 `guess`
        //  - Rust 允许使用同一个变量名重新绑定一个新值，这叫做 shadowing（隐藏）
        //  - 这里新的 `guess` 会隐藏前面那个字符串类型的 `guess`
        //  - `u32` 表示无符号 32 位整数，因为要猜的数字是正整数
        // `guess.trim()`: 去掉字符串开头和结尾的空白字符
        //  - 用户按下回车时，输入内容末尾通常会带有换行符
        //  - 如果不调用 `trim`，像 `"50\n"` 这样的字符串无法直接解析成数字
        // `.parse()`: 尝试把字符串解析成其他类型
        //  - 因为前面写了 `let guess: u32`，所以 Rust 会推断这里要解析成 `u32`
        //  - `parse` 返回一个 `Result`，解析成功是 `Ok(num)`，解析失败是 `Err(...)`
        // `match`: 根据 `parse` 返回的不同结果执行不同分支
        let guess: u32 = match guess.trim().parse() {
            // 如果解析成功，就取出 Ok 中包含的数字，并赋值给新的 `guess`
            Ok(num) => num,
            // 如果解析失败，说明用户输入的不是有效数字
            // `_` 表示忽略错误里面的具体内容
            // `continue` 会跳过本次循环剩下的代码，直接开始下一轮循环，让用户重新输入
            Err(_) => continue,
        };

        // 将用户猜的数字和秘密数字进行比较
        // `guess.cmp(&secret_number)`: 比较 `guess` 和 `secret_number` 的大小
        //  - `cmp` 方法返回一个 `Ordering` 枚举值
        //  - `Ordering::Less` 表示左边的值更小
        //  - `Ordering::Greater` 表示左边的值更大
        //  - `Ordering::Equal` 表示两个值相等
        // `&secret_number`: 传入的是 `secret_number` 的引用
        //  - `cmp` 方法需要接收一个引用作为参数
        //  - 使用引用可以让代码读取 `secret_number`，而不是取得它的所有权
        // `match`: 根据比较结果执行对应的提示逻辑
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                // `break` 会跳出当前 loop 循环，结束游戏
                break;
            }
        }
    }
}

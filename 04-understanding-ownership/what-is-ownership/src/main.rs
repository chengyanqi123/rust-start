fn main() {
    // 1.
    // 对于基本的数据类型。存在栈(Stack)上的数据
    //  - 进入作用域时，它就是有效的
    //  - 一直持续到它离开作用域为止
    {
        // s 在这里无效，它尚未声明
        let str = "hello"; // 从此处起，s 是有效的
    }
    // 此作用域已结束，s 不再有效

    // 2.
    // 一个指向存放字符串内容内存的指针，一个长度，和一个容量，这一引用组数据存储在栈上。
    // Stack:
    // s1                  s2
    // +-----+-----+-----+  +-----+-----+-----+
    // | ptr | len | cap |  | ptr | len | cap |
    // +--|--+-----+-----+  +--|--+-----+-----+
    //    |                    |
    //    +--------------------+
    //             |
    //             v
    // Heap:
    // +---+---+---+---+---+
    // | h | e | l | l | o |
    // +---+---+---+---+---+
    let s1 = String::from("hello");
    let s2 = s1;
    // s2 = s1 之后：
    // s1 被废弃，不再有效，所有权转移（move）。Rust 自动调用 `drop` 函数并清理变量的堆内存
    // s2 成为唯一所有者
    // println!("{s1}")    // ❌ 报错：value borrowed here after move
    // 此时只能通过 s2 访问堆上的 "hello"
    println!("{s2}");

    // 3.
    // 最初在堆上的 "hello" 将被切断引用。 s1 的指针重新指向堆上的 "ahoy"。
    // 完全没有任何内容指向堆上的 "hello"，Rust 会在其上运行 drop 函数同时内存会马上释放。
    // Stack:
    //         s1
    // +-----+-----+-----+
    // | ptr | len | cap |
    // +--|--+-----+-----+
    //    |
    //    +--------------------------+
    //    | x                        |
    //    v                          v
    // Heap:
    // +---+---+---+---+---+    +---+---+---+---+
    // | h | e | l | l | o |    | a | h | o | y |
    // +---+---+---+---+---+    +---+---+---+---+
    let mut s3 = String::from("hello");
    s3 = String::from("ahoy");
    println!("{s3}, world!");

    // 4.
    // 如果需要深度复制 String 中堆上的数据，而不仅仅是栈上的数据，可以使用 `clone` 方法。
    // 但是，使用 `clone` 方法进行深度复制可能相当消耗资源
    // Stack                         Heap
    // s1
    // +-----+-----+----------+       index  value
    // | ptr | len | capacity | ----> +---+---+---+---+---+
    // +-----+-----+----------+       | h | e | l | l | o |
    // |     |  5  |    5     |       +---+---+---+---+---+
    // +-----+-----+----------+

    // s2
    // +-----+-----+----------+       index  value
    // | ptr | len | capacity | ----> +---+---+---+---+---+
    // +-----+-----+----------+       | h | e | l | l | o |
    // |     |  5  |    5     |       +---+---+---+---+---+
    // +-----+-----+----------+
    let str1 = String::from("hello");
    let str2 = str1.clone();
    println!("str1 = {str1}, str2 = {str2}");

    // 5.
    // 带有所有权和作用域注释的函数
    let s = String::from("hello"); // s 进入作用域
    takes_ownership(s); // s 的值移动到函数里 ...
                        // ... 所以到这里不再有效
    let x = 5; // x 进入作用域
    makes_copy(x); // x 应该移动函数里，
                   // 但 i32 是 Copy 的，
    println!("{}", x); // 所以在后面可继续使用 x
} // 这里，x 先移出了作用域，然后是 s。但因为 s 的值已被移走，
  // 没有特殊之处

fn takes_ownership(some_string: String) {
    // some_string 进入作用域
    println!("{some_string}");
} // 这里，some_string 移出作用域并调用 `drop` 方法。
  // 占用的内存被释放

fn makes_copy(some_integer: i32) {
    // some_integer 进入作用域
    println!("{some_integer}");
} // 这里，some_integer 移出作用域。没有特殊之处

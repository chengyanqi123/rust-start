// docs @see https://kaisery.github.io/trpl-zh-cn/ch04-02-references-and-borrowing.html

fn main() {
    // 1. 引用（reference）和借用（borrowing）
    //  - 引用使用 & 操作符，与之相反的是解引用（dereferencing），操作符为 *
    //  - 创建一个引用的行为称为 借用
    let s1 = String::from("hello");
    let len = calculate_length(&s1); // 这里使用的引用 `&s1`, 看起来有点像一个指针，因为它是一个地址。
    println!("The length of '{s1}' is {len}."); // 此时 `s1` 并不会回收，可以正常访问了

    // 1.1 如果尝试修改借用的变量，这不可行。
    // 正如变量默认是不可变的，引用默认也是不可变的。不允许通过引用修改它指向的值。
    // change(&s1); // ❌ cannot borrow `*some_string` as mutable, as it is behind a `&` reference
    // change(&mut &s1); // ❌ cannot borrow data in a `&` reference as mutable

    // 2. 可变引用
    //  - 使用 `$mut` 来创建一个可变的引用
    //  - 被引用的值必须是可变的 `mut`
    let mut s2 = String::from("hello");
    change(&mut s2);
    println!("The changed after '{s2}' is {s2}.");
    // 2.1 可变引用有一个很大的限制：
    //  - 如果你有一个对该变量的可变引用，你就不能再创建对该变量的引用。它会在以下三种行为同时发生时出现：
    //      - 两个或更多指针同时访问同一数据。
    //      - 至少有一个指针被用来写入数据。
    //      - 没有同步数据访问的机制。
    // 这些尝试创建两个 s 的可变引用的代码会失败：
    // let mut s = String::from("hello");
    // let r1 = &mut s;
    // let r2 = &mut s;
    // println!("{r1}, {r2}");
    // ❌ cannot borrow `s` as mutable more than once at a time
    // 这个报错说明这段代码无效，因为我们不能在同一时间多次以可变方式借用 s。
    // 第一个可变借用在 r1 中，并且必须持续到它在 println! 中被使用；
    // 但在这个可变引用被创建和被使用之间，我们又尝试在 r2 中创建另一个可变引用，它借用的是和 r1 相同的数据。

    // ✅ 可以使用大括号来创建一个新的作用域，以允许拥有多个可变引用，只是不能同时拥有：
    let mut s = String::from("hello");
    {
        let r1 = &mut s;
    } // r1 在这里离开了作用域，所以我们完全可以创建一个新的引用
    let r2 = &mut s;

    // 2.2
    // 也不能在拥有不可变引用的同时拥有可变引用
    let r3 = &s;
    let r4 = &s;
    // let r5 = &mut s; // ❌ cannot borrow `s` as mutable because it is also borrowed as immutable
    // println!("{r3}, {r4}, and {r5}");
    // 不可变引用的借用者可不希望在借用时值会突然发生改变！
    // 然而，多个不可变引用是可以的，因为没有哪个只能读取数据的引用者能够影响其他引用者读取到的数据。
    // 注意一个引用的作用域从声明的地方开始一直持续到最后一次使用为止。
    // 例如，因为最后一次使用不可变引用的位置在 println!，它发生在声明可变引用之前，所以如下代码是可以编译的：
    let r5 = &mut s; // 没问题
    println!("{r5}");

    // 3. 悬垂引用（lifetimes）
    // 在带有指针的语言中，如果释放了一块内存，却保留了指向它的指针，就很容易错误地制造出一个悬垂指针（dangling pointer）：
    //  - 这个指针指向的内存位置可能已经被分配作其他用途。
    // 相比之下，在 Rust 中，编译器保证引用永远不会变成悬垂引用：
    //  - 如果你持有某些数据的引用，编译器会确保这些数据不会在它们的引用之前离开作用域。
    // let reference_to_nothing = dangle(); // ❌ &String -> expected named lifetime parameter

    // 总结：
    //  - 在任意给定时间，要么只能有一个可变引用，要么只能有多个不可变引用。
    //  - 引用必须总是有效的。
    // 此章节到此结束，下章节（slices）继续介绍不同类型的引用 `slice` 的概念。
}

// 获取一个字符的长度并返回:
//        s                          s1
// ┌──────────────┐      ┌──────────────────────┐
// │ name │ value │      │ name │ value         │
// ├──────┼───────┤      ├──────┼───────────────┤
// │ ptr  │   ────┼─────▶│ ptr  │          ─────┼──▶ ┌───┬───┬───┬───┬───┐
// └──────┴───────┘      ├──────┼───────────────┘    │ h │ e │ l │ l │ o │
//                       │ len  │ 5             │    └───┴───┴───┴───┴───┘
//                       ├──────┼───────────────┤
//                       │ cap  │ 5             │
//                       └──────┴───────────────┘
fn calculate_length(s: &String) -> usize {
    s.len()
}

// fn change(s: &String) {
//     s.push_str(", world");
// }
// 我们必须把 s 改成 &mut
fn change(s: &mut String) {
    s.push_str(", world");
}

// 这里 s 离开作用域并被丢弃。其内存被释放。
// 但是返回了字符串 s 的引用
fn dangle() -> &String {
    let s = String::from("hello");
    &s // ❌
       // 正确的做法是直接返回 s
}

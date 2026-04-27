fn main() {
    // let x = 5;
    // println!("The value of x is: {x}");
    // x = 6;   // >> error[E0384]: cannot assign twice to immutable variable `x`
    // println!("The value of x is: {x}");

    let mut x = 5;
    println!("The value of x is: {x}");
    x = 6; // ✅
    println!("The value of x is: {x}");

    // 常量使用 `const` 定义，命名规范采用 `大写 + '_'` 形式
    // @see https://doc.rust-lang.org/reference/const_eval.html
    const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;

    // 变量遮蔽：在同一个作用域下再次重复声明一个变量，最后声明的会覆盖之前声明的。
    // 常用于进行类型的转换
    let x = 5;
    let x = x + 1;
    {
        let x = x * 2;
        println!("The value of x in the inner scope is: {x}");
    }
    println!("The value of x is: {x}");
    // >> The value of x in the inner scope is: 12
    // >> The value of x is: 6
}

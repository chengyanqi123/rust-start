// cn-docs @see https://kaisery.github.io/trpl-zh-cn/ch03-02-data-types.html

fn main() {
    // 四种基本的标量类型
    //  - 整型：没有小数部分的数字。有符号整型以 `i` 开头，无符号整型以 `u` 开头
    //      - 默认值：Rust中，整型默认是 `i32`
    //      - 有符号 和 无符号 指的是数字是否可能为负数
    //      - 有符号的存储范围 -(2 ^ (n - 1)) 到 2 ^ (n - 1) - 1
    //      - 有符号的存储范围 0 到 2 ^ (n - 1)
    //      |-------|--------|----------|
    //        长度	   有符号     无符号
    //       8-bit	   i8	     u8
    //       16-bit	   i16	     u16
    //       32-bit	   i32	     u32
    //       64-bit	   i64	     u64
    //       128-bit   i128	     u128
    //       架构相关	isize     usize
    //      |-------|--------|----------|
    //     *isize 和 usize 类型依赖运行程序的计算机架构
    //      - 64 位架构上它们是 64 位的
    //      - 32 位架构上它们是 32 位的
    //      整型溢出
    //          - 在 `debug` 模式下，Rust 会加入整型溢出的检查，并在发生这种情况时让程序在运行时 `panic`
    //          - 在 `release` 模式下编译会出现 '溢出的回绕行为'。对于 u8 来说，256 会变成 0，257 会变成 1，依此类推。
    //      为了显式地处理溢出的可能性，可以使用这几类标准库提供的原始数字类型方法：
    //          - 所有模式下都可以使用 wrapping_* 方法进行 wrapping，如 wrapping_add
    //          - 如果 checked_* 方法发生溢出，则返回 None 值
    //          - 用 overflowing_* 方法返回值和一个布尔值，表示是否出现溢出
    //          - 用 saturating_* 方法在值的最小值或最大值处进行饱和处理
    let x: u8 = 10;
    let y: i32 = -128;

    //  - 浮点型：所有的浮点型都是有符号的，类型分为 `f32` 和 `f64`。默认类型是 `f64`。
    let difference = 95.5 - 4.3;
    let quotient = 56.7 / 32.2;
    let truncated = -5 / 3;

    //  - 布尔类型：用 `bool` 表示，有两个可能的值：`true` 和 `false`。
    let t = true;
    let f: bool = false; // with explicit type annotation

    //  - 字符类型：`char` 类型是语言中最原始的字母类型，使用单引号来表示 `char` 字面值。
    //      - Rust 的 char 类型大小为 4 个字节，并表示一个 Unicode 标量值（Unicode Scalar Value），
    //      - 它所能表示的内容远不止 ASCII。
    //      - 带重音符号的字母，中文、日文、韩文字符，emoji，以及零宽空格，都是 Rust 中合法的 char 值
    let c = 'z';
    let z: char = 'ℤ'; // with explicit type annotation
    let heart_eyed_cat = '😻';

    // 复合类型：可以把多个值组合成一个类型。
    // 有两种原生的复合类型：元组（tuple）和数组（array）

    // - 元组：元组是一种将多个不同类型的值组合成一个复合类型的通用方式。
    //        元组长度固定：一旦声明，它的大小就不能增长或缩小
    let mut tup: (i32, f64, u8) = (500, 6.4, 1);
    tup = (1024, 3.14, 8); // ✅
    // tup = (2048, 0.99); // ❌
    // tup = (6.18, 1, 2); // ❌
    // 解构元组
    let (x, y, z) = tup;
    println!("The value of y is: {y}");
    // 也可以使用点号（.）后跟值的索引来直接访问所需的元组元素
    let x: (i32, f64, u8) = (500, 6.4, 1);
    let five_hundred = x.0;
    let six_point_four = x.1;
    let one = x.2;
    // 不带任何值的元组有一个特殊名字，叫做 单元（unit）。
    // 这种值以及其对应的类型都写作 ()，表示空值或空的返回类型。
    // 如果一个表达式没有返回任何其他值，它就会隐式返回单元值。
    let unit_tup = ();

    // - 数组：数组长度是固定的，每个元素都必须具有相同类型。
    //        数组是在栈（stack）上分配的一整块、大小已知且固定的内存。
    // 在方括号中书写每个元素的类型，后跟分号，再后跟数组元素的数量
    let a: [i32; 5] = [1, 2, 3, 4, 5];
    // 也可以
    let a = [1, 2, 3, 4, 5];
    // 你还可以通过在方括号中指定初始值加分号再加元素个数的方式来创建一个每个元素都为相同值的数组：
    let a = [3; 5];
    // 等效于
    let a = [3, 3, 3, 3, 3];
    // 访问
    println!("The value of the element at index 2 is: {}", a[2]);
    // 下标越界
    // index out of bounds: the length is 5 but the index is 5
    // println!("The value of the element at index 2 is: {}", a[5]); // ❌
}

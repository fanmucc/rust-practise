# Rust 学习

## day1 基础类型

### 基础类型

#### 整数类型

| 长度   | 无符号（有正负之分）     | 有符号（无符号） |
| ------ | ------------------------ | ---------------- |
| 8-bit  | i8 -(2ⁿ - 1) 到 2ⁿ⁻¹ - 1 | u8 0 - 2ⁿ⁻¹ -1   |
| 16-bit | i16                      | u16              |
| 32bit  | i32                      | u32              |
| 64bit  | i64                      | u64              |
| 128bit | i128                     | u128             |
| arch   | isize                    | usize            |

`isize` 和 `usize` 类型

- `isize`和 `usize` 类型 由程序运行的计算机架构所决定，如果是 64 位则就位 64 位
- 主要场景是对某种集合进行索引操作

整数溢出情况:

如: `u8`的范围是 0-255, 如果把一个 `u8` 类型的值设为 256,那么：

- 在调试模式下编译：rust 会检查整数溢出，如果发生溢出，程序在运行时就会警告

![image.png](attachment:18370ac1-8e18-41ef-a6ab-6e09b3c7309c:image.png)

- 在发布模式下，rust 不会检查可能导致警告的整数溢出，而是会进行 `环绕操作` 即将: `256 -> 0` `257 -> 1`

```jsx
fn main() {
    println!("Hello, world!");

    // 无符号整数溢出示例
    let u8_value: u8 = 255;
    println!("u8 before overflow: {}", u8_value);
    let u8_overflow = u8_value + 1;
    println!("u8 after overflow: {}", u8_overflow); // 会输出 0

    // 有符号整数溢出示例
    let i8_value: i8 = 127;
    println!("i8 before overflow: {}", i8_value);
    let i8_overflow = i8_value + 1;
    println!("i8 after overflow: {}", i8_overflow); // 会输出 -128
}

╰─$ cargo run --release                                                                                                                                                                                 101 ↵
   Compiling rust-practise v0.1.0 (/Users/yan/personal/rust-practise)
    Finished `release` profile [optimized] target(s) in 0.53s
     Running `target/release/rust-practise`
Hello, world!
u8 before overflow: 255
u8 after overflow: 0
i8 before overflow: 127
i8 after overflow: -128
```

#### 浮点类型

| 类型  | 大小  | 精度                 | 示例                 |
| ----- | ----- | -------------------- | -------------------- |
| `f32` | 32 位 | 单精度浮点数         | `let x: f32 = 3.14;` |
| `f64` | 64 位 | 双精度浮点数（默认） | `let x = 2.718;`     |

![image.png](attachment:2745a4be-21be-404a-936a-25ff17177e2d:image.png)

#### 布尔类型

`let t: bool = true;`
`let f: bool = false;`

#### 字符类型

`char`基础单个字符

- 字符类型的字面值使用单引号
- 占用 4 字节大小
- 是 `Unicode` 标量值，可以表示比 `ASCLL` 多得多的字符内容，如：拼音、中日韩人、零长度空白字符、emoji 表情等。

#### 单元类型（无值）：`()`

单元类型  ()  在 Rust 中有几个重要的用途：

1. 函数返回值：

- 当函数不需要返回任何值时，返回类型就是  `()`
- 例如：`fn main → ()` 或简写为  `fn main()`

1. 表示"无值"：

- 在泛型编程中，当某个类型参数不需要具体值时
- 在  `Option<T>`  中，None 实际上就是  `Option<()>`

1. 表示"无操作"：

- 在闭包中，当不需要执行任何操作时
- 例如：`let no_op = || ();`

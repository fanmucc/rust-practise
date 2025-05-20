fn main() {
    println!("Hello, world!");

    // 无符号整数溢出示例
    let u8_value: u8 = 255;
    println!("u8 before overflow: {}", u8_value);
    // let u8_overflow = u8_value + 1;
    // println!("u8 after overflow: {}", u8_overflow); // 会输出 0

    // 有符号整数溢出示例
    let i8_value: i8 = 127;
    println!("i8 before overflow: {}", i8_value);
    // let i8_overflow = i8_value + 1;
    // println!("i8 after overflow: {}", i8_overflow); // 会输出 -128

    // 浮点数
    let fx = 6.24; // f64
    let fy: f32 = 6.24; // f32
    println!("fx: {}", fx);
    println!("fy: {}", fy);

    // 布尔类型
    let ffloat: bool = false;
    println!("ffloat: {}", ffloat);

    // 字符类型
    let c: char = 'a';
    println!("c: {}", c);

    // 单元类型
    let unit: () = ();
    println!("unit: {:?}", unit);

    // 1. 函数返回值示例
    fn no_return() -> () {
        println!("This function returns nothing");
    }
    no_return();

    // 2. 表示"无值"示例
    let none: Option<()> = None;
    println!("none: {:?}", none);

    // 3. 表示"无操作"示例
    let no_op = || ();
    no_op();

    // 元组类型 Tuple类型
    /*
     * 元组类型是固定长度的，一旦声明，其长度不能改变。
     * 元组类型可以包含不同类型的元素，元素之间用逗号分隔。
     * 元组类型可以嵌套，即元组中可以包含另一个元组。
     * 元组类型可以为空，即没有元素的元组。
     */
    let tuple: (i32, f64, char) = (1, 2.0, 'a');
    // 解构赋值
    let (x, y, z) = tuple;
    // 嵌套 tuple
    let nested_tuple: (i32, (f64, char)) = (1, (2.0, 'a'));
    let (a, (b, c)) = nested_tuple;
    println!("nested_tuple: {} {} {}", a, b, c);
    // 空 tuple
    let empty_tuple: () = ();
    println!("empty_tuple: {:?}", empty_tuple);
    // 访问元组中的元素 .标记法
    println!("tuple: {} {} {}", tuple.0, tuple.1, tuple.2);
    println!("x: {} y: {} z: {}", x, y, z);

    // 数组类型
    /*
     * 数组类型是固定长度的，一旦声明，其长度不能改变。
     * 数组类型可以包含不同类型的元素，元素之间用逗号分隔。
     * 数组类型可以嵌套，即数组中可以包含另一个数组。
     * 数组类型可以为空，即没有元素的数组。
     */
    let arr: [i32; 3] = [1, 2, 3];
    // 解构赋值
    let [a, b, c] = arr;
    println!("arr: {:?}", arr);
    println!("a: {} b: {} c: {}", a, b, c);

    // 数组类型可以包含不同类型的元素，元素之间用逗号分隔。
    let arr2: [i32; 3] = [1, 2, 3];
    println!("arr2: {:?}", arr2);
    // 数组类型可以嵌套，即数组中可以包含另一个数组。
    let arr3: [[i32; 2]; 2] = [[1, 2], [3, 4]];
    let [arr3_a, arr3_b] = arr3[0];
    println!("arr3: {} {}", arr3_a, arr3_b);
    // 数组类型可以为空，即没有元素的数组。
    let empty_arr: [i32; 0] = [];
    println!("empty_arr: {:?}", empty_arr);

    // 切片类型
    let slice: &[i32] = &arr[0..2];
    println!("slice: {:?}", slice);
}

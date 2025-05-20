// 基本类型示例
pub fn run_basic_types() {
    // 整数类型
    let u8_value: u8 = 255;
    println!("u8 before overflow: {}", u8_value);

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

    // 数组和切片
    let arr: [u32; 5] = [1, 2, 3, 4, 5];
    let slice1: &[u32] = &arr[1..4]; // 从下标1开始，切到第4位（不包含第4位）
    println!("数组切片: {:?}", slice1); // 数组切片: [2, 3, 4]

    // 获取整个数组的切片
    let full_slice: &[u32] = &arr[..]; // 整个数组的切片
    println!("整个数组的切片: {:?}", full_slice);
}

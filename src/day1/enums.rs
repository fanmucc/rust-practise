// 枚举示例
pub fn run_enums() {
    // 1. 基本枚举
    #[derive(Debug)]
    enum Direction {
        North,
        South,
        East,
        West,
    }

    // 使用所有方向
    let directions = [
        Direction::North,
        Direction::South,
        Direction::East,
        Direction::West,
    ];

    // 打印所有方向
    for direction in directions.iter() {
        println!("Direction: {:?}", direction);
    }

    // 2. 带数据的枚举
    #[derive(Debug)]
    enum Message {
        Quit,
        Move { x: i32, y: i32 },
        Write(String),
        ChangeColor(i32, i32, i32),
    }

    // 创建枚举实例
    let msg1 = Message::Quit;
    let msg2 = Message::Move { x: 10, y: 20 };
    let msg3 = Message::Write(String::from("Hello"));
    let msg4 = Message::ChangeColor(255, 0, 0);

    println!("Message 1: {:?}", msg1);
    println!("Message 2: {:?}", msg2);
    println!("Message 3: {:?}", msg3);
    println!("Message 4: {:?}", msg4);

    // 3. 枚举方法
    impl Message {
        // 实例方法：使用 &self 参数
        // &self 表示这个方法可以访问枚举实例
        fn call(&self) {
            match self {
                Message::Quit => println!("Quit message"),
                Message::Move { x, y } => println!("Move to x: {}, y: {}", x, y),
                Message::Write(text) => println!("Text message: {}", text),
                Message::ChangeColor(r, g, b) => {
                    println!("Change color to RGB({}, {}, {})", r, g, b)
                }
            }
        }

        // 静态方法：不使用 &self 参数
        // 可以直接通过类型调用，不需要实例
        fn new_quit() -> Message {
            Message::Quit
        }

        fn new_move(x: i32, y: i32) -> Message {
            Message::Move { x, y }
        }
    }

    // 调用实例方法
    msg1.call(); // 通过实例调用
    msg2.call();
    msg3.call();
    msg4.call();

    // 调用静态方法
    let new_quit = Message::new_quit(); // 通过类型调用
    let new_move = Message::new_move(5, 5);
    println!("New quit message: {:?}", new_quit);
    println!("New move message: {:?}", new_move);
}

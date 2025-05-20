// 结构体示例
pub fn run_structs() {
    // 1. 基本结构体
    #[derive(Debug)]
    struct Person {
        name: String,
        age: u32,
        email: String,
    }

    // 方法1：实现 Default trait
    impl Default for Person {
        fn default() -> Self {
            Person {
                name: String::from("Unknown"),
                age: 0,
                email: String::from("unknown@example.com"),
            }
        }
    }

    // 方法2：使用构造函数
    impl Person {
        fn new() -> Self {
            Person {
                name: String::from("Unknown"),
                age: 0,
                email: String::from("unknown@example.com"),
            }
        }

        // 带参数的构造函数
        fn with_name(name: String) -> Self {
            Person {
                name,
                age: 0,
                email: String::from("unknown@example.com"),
            }
        }
    }

    // 使用默认值创建实例
    let default_person = Person::default();
    println!("Default person: {:?}", default_person);

    // 使用构造函数创建实例
    let new_person = Person::new();
    println!("New person: {:?}", new_person);

    // 使用带参数的构造函数
    let named_person = Person::with_name(String::from("Alice"));
    println!("Named person: {:?}", named_person);

    // 方法3：使用 #[derive(Default)] 派生宏
    #[derive(Debug, Default)]
    struct SimplePerson {
        name: String,  // String 已经实现了 Default
        age: u32,      // u32 已经实现了 Default
        email: String, // String 已经实现了 Default
    }

    // 使用默认值创建实例
    let simple_person = SimplePerson::default();
    println!("Simple person: {:?}", simple_person);

    // 部分字段使用默认值
    let custom_person = SimplePerson {
        name: String::from("Bob"),
        ..Default::default() // 其他字段使用默认值
    };
    println!("Custom person: {:?}", custom_person);

    // 2. 元组结构体
    #[derive(Debug)]
    struct Point(i32, i32);
    let point = Point(10, 20);
    println!("Point: {:?}", point);
    println!("x: {}, y: {}", point.0, point.1);

    // 3. 单元结构体
    #[derive(Debug)]
    struct Unit;
    let unit = Unit;
    println!("Unit: {:?}", unit);

    // 4. 结构体方法
    impl Person {
        // 实例方法
        fn introduce(&self) {
            println!("Hi, I'm {}, {} years old.", self.name, self.age);
        }
    }

    let person2 = Person::new();
    person2.introduce();
}

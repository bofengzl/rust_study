pub fn add(left: usize, right: usize) -> usize {
    left + right
}

pub fn add_two(a: i32) -> i32 {
    a + 2
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }

    #[test]
    fn exploration() {
        assert_eq!(2 + 2, 4)
    }

    #[test]
    fn another() {
        panic!("Exception.")
    }

    #[derive(Debug)]
    struct Rectangle {
        width: u32,
        height: u32,
    }

    impl Rectangle {
        fn can_hold(&self, other: &Rectangle) -> bool {
            return self.width > other.width && self.height > other.height;
        }
    }

    // 使用 assert!() 宏来检查结果 ， true → 正常编译、 false → 触发panic!()
    #[test]
    fn larger_can_hold_fn() {
        let rect1 = Rectangle {
            width: 10,
            height: 7,
        };
        let rect2 = Rectangle {
            width: 5,
            height: 5,
        };
        assert!(rect1.can_hold(&rect2))
    }

    #[test]
    fn larger_cannot_hold_fn() {
        let rect1 = Rectangle {
            width: 10,
            height: 7,
        };
        let rect2 = Rectangle {
            width: 15,
            height: 15,
        };
        assert!(rect1.can_hold(&rect2))
    }

    // 使用 assert_eq!() 和 assert_ne!() 来测试 是否相等
    // assert_eq!()：两个值相等[==] → 正常编译， 反之 → panic!()
    // assert_ne!()：两个值不相等[!=] → 正常编译，反之 → panic!()

    // 自定义失败信息
    fn greeting(name: &str) -> String {
        format!("Hello {}!", name)
    }

    #[test]
    fn greeting_can() {
        let result = greeting("Carol");
        assert!(result.contains("Carol"));
    }

    // #[warn(unused_variables)]
    fn greetings(_name: &str) -> String {
        format!("Hello!")
    }

    #[test]
    fn greeting_cannot() {
        let result = greetings("Test");
        // 自定义失败信息
        assert!(
            result.contains("Test"),
            "Greeting did not contains name, value was `{}`",
            result
        );
    }

    // #[should_panic] 用于检查 panic
    pub struct Guess {
        value: i32,
    }

    impl Guess {
        pub fn new(value: i32) -> Guess {
            if value < 1 || value > 100 {
                panic!("Guess the value must between 1 and 100. current value: {}", value)
            }
            Guess { value }
        }
    }

    #[test]
    #[should_panic(expected = "less than or equal to 100")]
    fn get_guess_100(){
        
        let guess = Guess::new(10);
        println!("value {}", guess.value)

    }

    // Result<T, E> 用于测试
    #[test]
    fn is_works() -> Result<(), String> {
        if 2 + 2 == 4 {
            Ok(())
        } else {
            Err(String::from("error"))
        }
    }
}

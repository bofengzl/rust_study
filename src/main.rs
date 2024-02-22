// ********** Hello ferris_says! 测试编程 START**********

// use ferris_says::say; // from the previous step
// use std::io::{stdout, BufWriter};

// fn main() {
//     let stdout = stdout();
//     let message = String::from("Hello ferris_says!");
//     let width = message.chars().count();

//     let mut writer = BufWriter::new(stdout.lock());
//     // as_bytes() → 将 String 转成字节切片
//     say(message.as_bytes(), width, &mut writer).unwrap();
// }

// ********** Hello ferris_says! 测试编程 END**********

// ********** 猜数字DEMO START**********

// use rand::Rng; // Rng 是一个 trait，它定义了随机数生成器应实现的方法
// use std::cmp::Ordering;
// use std::io; // 引入 io输入输出库 当前作用域
// fn main() {
//     println!("猜猜数字游戏");
//     let random_num = rand::thread_rng().gen_range(1..=100);
//     loop {
//         println!("请输入你的猜测。");
//         // let guess = 5 变量不可变
//         let mut guess = String::new(); // 加上 mut → 变量可变
//         io::stdin().read_line(&mut guess).expect("读取失败!");
//         // 定义同名变量 会将之前定义的隐藏掉， 产生新的变量
//         let guess: u32 = match guess.trim().parse() {
//             Ok(num) => num, // 正确类型就返回当前值
//             Err(_) => {
//                 println!("Please input type a number!");
//                 continue;
//             }, // 错误类型 进行下一次循环
//         };
//         println!("你猜的：{guess}");
//         match guess.cmp(&random_num) {
//             Ordering::Less => println!("太小了！"),
//             Ordering::Greater => println!("太大了！"),
//             Ordering::Equal => {
//                 println!("猜对了！");
//                 break;
//             }
//         }
//     }
// }

// ********** 猜数字DEMO END**********

// ********** 基础概念 START**********

// 变量和可变性
// fn main (){
//     let mut x = 5;
//     println!("The value of x is: {x}");
//     x = 8;
//     println!("The value of x is: {x}");
// }

// 影藏
// fn main () {
//     let x = 5;
//     let x = x + 1;
//     {
//         let x = x * 2;
//         // 此时访问的作用域内的 x
//         println!("The value of  x in the inner scope is: {x}")
//     }
//     // 访问的是 第二次 重新赋值的变量 x
//     println!("The value of x is: {x}");

//     // let mut spaces = "   ";
//     let  spaces = "   ";
//     let spaces = spaces.len();
//     println!("The value of spaces is: {spaces}")
// }

// 定义元组及
// fn main () {
//   let tup: (u32, f64, u8) = (400, 1.8, 10);
//   let (x, y, z) = tup;
//   println!("The value of x is: {x}");
//   println!("The value of y is: {y}");
//   println!("The value of z is: {z}");
// }

// 访问无效数据元素
// use std::io;

// fn main () {
//     let a = [1, 2, 3, 4, 5];
//     println!("Please enter an array index.");
//     let mut index = String::new();
//     io::stdin()
//         .read_line(&mut index)
//         .expect("Failed to read line.");

//     let index: usize = index
//         .trim()
//         .parse()
//         .expect("index entered was not number.");
//     let element = a[index];
//     println!("The value of the element at index {index} is: {element}");
// }

// 定义函数

// fn main () {
//   println!("Hello World!");
//   another_function();
//   another_params(10);
// }

// fn another_function () {
//   println!("another_function");
// }

// // 参数
// fn another_params (x: u32) {
//   println!("The value of x is: {x}")
// }

// fn plus_one(x: i32) -> i32 {
//   x + 1
//   // x + 1;
// }

// fn main () {
//   let x = plus_one(10);
//   println!("The value of x is: {x}");
// }

// fn main() {
//   let number = 6;

//   if number % 4 == 0 {
//       println!("number is divisible by 4");
//   } else if number % 3 == 0 {
//       println!("number is divisible by 3");
//   } else if number % 2 == 0 {
//       println!("number is divisible by 2");
//   } else {
//       println!("number is not divisible by 4, 3, or 2");
//   }
// }

// fn main () {
//   let mut count = 0;
//   'counting_up: loop {
//       println!("count = {count}");
//       let mut remaining = 10;
//       loop {
//           println!("remaining = {remaining}");
//           if remaining == 9 {
//             // 终止内层循环
//             break;
//           }
//           if count == 2 {
//             // 终止外层循环
//             break 'counting_up;
//           }
//           remaining -= 1;
//       }
//       count += 1;
//   }
//   println!("END count = {count}");
// }

// fn main () {
//   let mut number = 3;
//   // 条件为 true 输出 number \ 反之 LIFTOFF!
//   while number != 0 {
//       println!("{number}!");
//        number -= 1;
//   }
//   println!("LIFTOFF!");
// }

// fn main () {
//   let list = [10, 20, 30, 40, 50];
//   let mut index = 0;
//   while index < list.len() {
//       println!("The value of list[{index}] is: {}", list[index]);
//       index += 1;
//   }
// }

// fn main () {
//   let list = [10, 20, 30, 40, 50];
//   for item in list {
//     println!("The value of item is: {item}");
//   }
// }

// 倒计时例子
// fn main() {
//   // rev，用来反转
//   for number in (1..4).rev() {
//       println!("{number}!");
//   }
//   println!("LIFTOFF!!!");
// }

// ********** 基础概念 END**********

// ********** 所有权 START**********

// fn main () {
//   let s1 = String::from("hello");
//   // let s2 = s1.clone();
//   // println!("{s2}");
//   one(s1);

//   let s3 = 10;
//   two(s3);

//   let a = [1, 2, 3, 4, 5];
//   let slice = &a[1..3];
//   assert_eq!(slice, &[2,3]); // 断言成功 → 程序继续往下执行， 断言失败 → 抛出错误
//   println!("The value of slice is: {} {}",slice.len(),slice[1]);
// }

// fn one (some_string: String) {
//   println!("{some_string}")
// }
// fn two (some_integer: u32) {
//   println!("{some_integer}")
// }
// ********** 所有权 END************

// ********** 结构体组织相关联的数据 START**********

// #[derive(Debug)]
// struct User {
//   active: bool,
//   username: String,
//   email: String,
//   sing_in_count: u64,
// }
// #[derive(Debug)]
// struct Color(i32, i32, i32);

// #[derive(Debug)]
// struct Point(i32, i32, i32);

// #[derive(Debug)]
// struct Rectangle {
//   width: u32,
//   height: u32,
// }

// impl Rectangle {
//   fn area(&self) -> u32 {
//     self.height * self.width
//   }
//   // 可以选择将方法的名称与结构中的一个字段相同。
//   fn width(&self) -> bool {
//     self.width > 0
//   }
//   // 正方形
//   fn square(size: u32) -> Self {
//     Self {
//       width: size,
//       height: size,
//     }
//   }
// }


// fn main() {
//   let user1 = User {
//     active: true,
//     username: String::from("123"),
//     email: String::from("1234@example.com"),
//     sing_in_count: 1,
//   };
//   // println!("{:?}", user1); 输出复杂的数据
//   println!("The value of user1 is: {:?}", user1);

//   // let user2 = User {
//   //   active: user1.active,
//   //   username: user1.username,
//   //   email: String::from("user2@example.com"),
//   //   sing_in_count: user1.sing_in_count,
//   // };

//   // 简化写法
//   let user3 = User {
//     email: String::from("another@example.com"),
//     ..user1
//   };
//   println!("The value of user3 is: {:#?}", user3); // 可格式化 输出json

//   let black = Color(0,0,0);
//   println!("The value of black is: {:?}", black); 
//   let origin = Point(0,0,0);
//   println!("The value of origin is: {:?}", origin);

//   // 方法语法
//   let rect1 = Rectangle {
//     width: 100,
//     height: 10,
//   };

//   println!("The area of rectangle is {} square pixels.", rect1.area());

//   if rect1.width() {
//     println!("The rectangle has a nonzero width; it is {}", rect1.width);
//   }

//   let sq = Rectangle::square(3);
//   println!("The value of sq is: {:?}", sq);

// }

// fn build_user(email: String, username: String) -> User {
//     // 简化语法
//     User {
//         active: true,
//         username,
//         email,
//         sing_in_count: 1,
//     }

//     // User {
//     //   active: true,
//     //   username: username,
//     //   email: email,
//     //   sing_in_count: 1
//     // }
// }

// ********** 结构体组织相关联的数据 END**********

// ********** 枚举和模式匹配 START**********

// enum IpAddrKind {
//   // V4,
//   // V6,
//   // 简化写法
//   V4(String),
//   V6(String),
//   V7(u8, u8, u8, u8)
// }

// // struct IpAddr {
// //   kind: IpAddrKind,
// //   address: String,
// // }

// fn main() {
//   // let four = IpAddrKind::V4;
//   // let five = IpAddrKind::V6;

//   // route(IpAddrKind::V4);
//   // route(IpAddrKind::V6);

//   // let local = IpAddr {
//   //   kind: IpAddrKind::V4,
//   //   address: String::from("127.0.0.1"),
//   // };

//   // let test = IpAddr {
//   //   kind: IpAddrKind::V6,
//   //   address: String::from("::1"),
//   // };
  

//   let home = IpAddrKind::V4(String::from("127.0.0.1"));
//   let test = IpAddrKind::V6(String::from("::1"));
//   let test1 = IpAddrKind::V7(127, 0, 0, 1);

//   let enter_value = value_in_enter(Coin::Quarter(UsState::Alabama));
//   println!("The value of enter is: {}", enter_value) // The value of enter is: 100

// }

// fn route(ip: IpAddrKind) {

// }

// #[derive(Debug)]
// enum UsState {
//   Alabama,
//   Alaska,
// }
// enum Coin {
//   Penny,
//   Nickel,
//   Dime,
//   Quarter(UsState),
// }

// fn value_in_enter(coin: Coin) -> i8 {
//   match coin {
//     Coin::Quarter(state) => {
//       println!("The value of state is: {:?} !", state); // The value of state is: Alabama !
//       2
//     },
//     Coin::Penny => 5,
//     Coin::Nickel => 50,
//     Coin::Dime => 100,
//   }
// }
#[derive(Debug)]
enum UsState {
    Alabama,
    Alaska,
    // --snip--
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

fn main() {
    let coin = Coin::Quarter(UsState::Alabama);
    let mut count = 0;
    if let Coin::Quarter(state) = coin {
        println!("State quarter from {:?}!", state);
    } else {
        count += 1;
        println!("count {:?}!", count);
    }
}
// ********** 枚举和模式匹配 END**********

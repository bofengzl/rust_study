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
// #[derive(Debug)]
// enum UsState {
//     Alabama,
//     Alaska,
//     // --snip--
// }

// enum Coin {
//     Penny,
//     Nickel,
//     Dime,
//     Quarter(UsState),
// }

// fn main() {
//     let coin = Coin::Quarter(UsState::Alabama);
//     let mut count = 0;
//     if let Coin::Quarter(state) = coin {
//         println!("State quarter from {:?}!", state);
//     } else {
//         count += 1;
//         println!("count {:?}!", count);
//     }
// }
// ********** 枚举和模式匹配 END**********

// ********** 使用包、Crate 和模块管理不断增长的项目 START**********

// // 定义模块来控制作用域与私有性
// use crate::garden::vegetables::Asparagus;

// pub mod garden;

// // use restaurant;

// fn main() {
//   let plant = Asparagus {
//   };
//   println!("The value of plant is: {:?} !", plant);
// }

// ********** 使用包、Crate 和模块管理不断增长的项目 END**********

// ********** 常见集合 START**********

// fn main() {
//   // ----------使用 Vector 储存列表-----------

//   // 使用 Vec::new() 创建
//   let mut v1: Vec<i32> = Vec::new();
//   println!("The value of v1 is: {:?}", v1);

//   // 使用 vec! 宏创建
//   let v2 = vec![1, 2, 3];
//   println!("The value of v2 is: {:?}", v2);

//   // 更新 Vector
//   v1.push(1);
//   v1.push(2);
//   v1.push(3);
//   v1.push(4);
//   println!("The update value of v1 is: {:?}", v2);

//   // 获取 Vector 数据

//   // 方式一：通过下标的方式获取 [主]
//   let one = &v1[0];
//   println!("The value of one is: {:?}", one);

//   // 方式二：通过 get  [主]
//   let two = v1.get(1);
//   match two {
//       Some(two) => println!("There two el is {two}"),
//       None => println!("There is no two el"),
//   }

//   // 遍历 vector 中的元素
//   for i in &v1 {
//     println!("The value of i is: {}", i);
//   }

//   // 也可以遍历的同时改变数据
//   for i in &mut v1 {
//     *i += 50;
//     dbg!(i); // 会输出结果和代码位置信息
//   }
//   println!("The for in value v1 is: {:?}", v1);

//   // 使用枚举来储存多种类型
//   #[derive(Debug)]
//   enum Map {
//     Int(i32),
//     Float(f64),
//     Text(String),
//     Bool(bool)
//   }

//   let v3 = vec![
//     Map::Int(1),
//     Map::Float(10.11),
//     Map::Text(String::from("Test")),
//     Map::Bool(false)
//   ];
//   println!("The value of v3: is: {:#?}", v3);

//   // -------使用字符串储存 UTF-8 编码文本------------

//   // 使用 String::new() 创建
//   let mut s1 = String::new();
//   println!("The value of s1 is: {s1}");

//   // to_string() 转成 string
//   let int_str = "init contents";
//   let s2 = int_str.to_string(); // 也可以 "init contents".to_string() 或者 String::from("init contents") 等价于 to_string() 方式创建
//   println!("The value of s2 is: {s2}");

//   // 更新字符串
//   s1.push_str("test");
//   println!("The update value of s1 is: {s1}");

//   let s3 = "bar";
//   s1.push_str(s3);
//   println!("The value of s3 is: {s3}");

//   s1.push('l');
//   println!("The value of s1 is: {s1}");

//   // 使用 + 运算符或 format! 宏拼接字符串
//   let s4 = String::from("hello,");
//   let s5 = String::from(" word!");
//   let s6 = s4 + &s5; // 注意 后续 s4 被移动了，不可使用
//   println!("The value of s6 is: {s6}");

//   let s7 = format!("{s5} - {s6}"); // format!() 宏 使用别的变量是 引用所以获取任何数据的所有权，后续可继续使用
//   println!("The value of s6 is: {s7}");

//   // 字符串 slice
//   let s8 = "Здравствуйте";
//   let s9 = &s8[0..4];
//   println!("The value of s9 is: {s9}");

//   // 遍历字符串的方法

//   // 方法一 ： chars() 会将每个拆分成 单个的 char 类型的值
//   for i in s8.chars() {
//     dbg!(i);
//   }

//   // 方法二 as_bytes() 方式 Unicode 标量值
//   for i in s8.as_bytes() {
//     dbg!(i);
//   }

//   // string 方法 replace
//   println!("The replace value of s5 is: {}", s5.replace("word", "6"));

//   // --------------使用 Hash Map 储存键值对---------------------

//   //引入 HashMap
//   use std::collections::HashMap;
//   // HashMap::new() 创建
//   let mut h1 = HashMap::new();

//   // insert 插入元素

//   h1.insert(String::from("blue"), 1);
//   h1.insert(String::from("red"), 2);

//   println!("the value of h1 is: {:?}", h1);

//   // 访问哈希 map 的值：get()
//   let key1 = String::from("blue");
//   // get() 获取数据 返回的数据类型是 Option<&T>、 copied() 获取一个Option(i32)、unwrap_or(): key1 在 h1 中不存在时, get()会返回 None, 这个时候函数设置成默认值 0 即返回 0;
//   let value1 = h1.get(&key1).copied().unwrap_or(0);
//   println!("the value of value1 is: {}", value1);

//   // 也可以与 vector 类似的方式进行遍历 map
//   for (key, value) in &h1 {
//     println!("{key} :{value}");
//   }

//   // 哈希 map 和所有权
//   // 注：对于像 i32 这样的实现了 Copy trait 的类型，其值可以拷贝进哈希 map。对于像 String 这样拥有所有权的值，其值将被移动而哈希 map 会成为这些值的所有者
//   let field_name = String::from("Favorite color");
//   let field_value = String::from("Blue");
//   let number_name = String::from("number");
//   let number = 1;
//   let mut map = HashMap::new();
//   map.insert(field_name, field_value);
//   map.insert(number_name, number.to_string());

//   // 解除以下注释，会报错
//   // println!("{field_name}");
//   println!("{number}");

//   // 更新 哈希map
//   // 情况一 已存在key 重新再次插入 即可更新对应的值
//   h1.insert(String::from("blue"), 50);
//   println!("h1 update blue is: {:?}", h1);

//   // 情况二 只在键没有对应值时插入键值对
//   h1.entry(String::from("blue")).or_insert(10); // 存在建和值 所以不进行跟更新
//   h1.entry(String::from("yellow")).or_insert(50); // 不存在，所以更新
//   println!("entry or_insert fun is: {:#?}",h1);

//   // 情况三 根据旧值更新值
//   let text = "hello world wonderful world";
//   let mut h2 = HashMap::new();
//   for word in text.split_whitespace() {
//     // split_whitespace 方法返回一个由空格分隔 text 值子 slice 的迭代器。 or_insert(0)：返回的是可变引用（&mut v） 所以得使用 * 解引
//     let count  = h2.entry(word).or_insert(0);
//     *count += 1;
//   }
//   println!("The value of h2 is: {:?}", h2);

// }

// ********** 常见集合 END**********

// ********** 错误处理 START**********

// 总体分类：可恢复的（recoverable）和 不可恢复的（unrecoverable）错误

// // use std::env;
// use std::io;
// use std::{
//     fs::{self, File},
//     io::{ErrorKind, Read},
// };

// #[derive(Debug)]
// pub struct Guess {
//   value: i32,
// }

// impl Guess {
//   pub fn new(value: i32) -> Guess {
//       if value < 1 || value > 100 {
//           panic!("Guess value must be between 1 and 100, got {}.", value);
//       }
//       Guess { value }
//   }
//   pub fn value(&self) -> i32 {
//       self.value
//   }
// }
// fn main() {
//     // // 设置环境变量 RUST_BACKTRACE 来显示回溯
//     // env::set_var("RUST_BACKTRACE", "1");

//     // // 使用 panic! 处理不可恢复的错误
//     // panic!("crash an burn");

//     // 使用Result处理可恢复的错误

//     // enum Result<T, E> {
//     //   Ok(T),
//     //   Err(E),
//     // }

//     // let file_info_result = File::open("hello.txt");
//     // let file_info = match file_info_result {
//     //     Ok(file) => file,
//     //     Err(error) => match error.kind() {
//     //       // ErrorKind::NotFound → 表示打开不存在的文件
//     //       // File::create() → 创建文件
//     //       ErrorKind::NotFound => match File::create("hello.txt") {
//     //           Ok(fc) => fc,
//     //           Err(e) => panic!("Problem creating the file: {:?}", e),
//     //       },
//     //       other_error => panic!("Problem opening the file: {:?}", other_error),
//     //     },
//     // };

//     // // 以上例子 使用 闭包和unwrap_or_else
//     // let file_info_closure = File::open("hello_closure.txt").unwrap_or_else(|error| {
//     //   if error.kind() == ErrorKind::NotFound {
//     //     File::create("hello_closure.txt").unwrap_or_else(|error| {
//     //       panic!("Problem creating the file: {:?}", error);
//     //     })
//     //   } else {
//     //     panic!("Problem opening the file: {:?}", error);
//     //   }
//     // });

//     // 失败时panic的简写： unwrap 和 expect
//     // let file_info_unwrap = File::open("err.txt").unwrap(); // unwrap() 会返回 ok 的值, 如果是 Result 是 Err , unwrap() 会自动调 panic!()
//     // let file_info_expect = File::open("err.txt").expect("err.txt should be included in this project."); // expect 在调用 panic! 时使用的错误信息将是我们传递给 expect 的参数，而不像 unwrap 那样使用默认的 panic! 信息。

//     // 传播错误
//     // let file_content = read_username_from_file();
//     // println!("The value of file_content is: {:?}", file_content);

//     let guess = Guess::new(10);
//     println!("The value of guess is:{:?}", guess.value);

// }

// // fn read_username_from_file() -> Result<String, io::Error> {
// // let username_file_result: Result<File, io::Error> = File::open("hello.txt");
// // let mut username_file = match username_file_result {
// //     Ok(file) => file,
// //     Err(e) => return Err(e),
// // };
// // let mut username = String::new();
// // match username_file.read_to_string(&mut username) {
// //     Ok(_) => Ok(username),
// //     Err(e) => Err(e),
// // }

// // 简化写法
// // let mut username_file = File::open("hello.txt")?;
// // let mut username = String::new();
// // username_file.read_to_string(&mut username)?;
// // Ok(username)

// // 简化写法： ? 问号运算符之后的链式方法调用
// // let mut username = String::from("");
// // File::open("hello.txt")?.read_to_string(&mut username)?;
// // Ok(username)

// //  使用 fs::read_to_string 而不是打开后读取文件
// // fs::read_to_string("hello.txt")
// // }

// ********** 错误处理 END**********

// ********** 泛型、Trait、生命周期 START**********


// use std::fmt::{Display, Debug};

// use rust_study::{Summary, Tweet};

// fn main() {
//   let tweet = Tweet {
//     username: String::from("house_ebooks"),
//     content: String::from("of course, as you probably already know, people."),
//     reply: false,
//     retweet: true,
//   };

//   println!("1 new tweet {}", tweet.size());
//   println!("1 new tweet {}", tweet.summary_fn());

//   // notify(&tweet);

//   // let pair = Pair::new(10, 100);
//   // let s = 3.to_string();
//   // Pair::cmp_display(&pair);

//   let string1 = String::from("abcd");
//   let string2 = "xyz";
//   let result = longest(string1.as_str(), string2);
//   println!("The longest string is {}", result);

//   let novel = String::from("Call me Ishmael. Some years age....");
//   let first_string = novel.split('.').next().expect("The Value of novel don't '.'");
//   let i = ImportantExcerpt {
//     part: first_string
//   };
//   println!("The value of i: {:#?}", i.part);
// }

// // pub fn notify(item: &impl Summary) {
// //   println!("Breaking news! {}", item.size());
// // }

// // pub fn notify<T: Summary>(item: &T) {
// //   println!("Breaking news! {}", item.size());
// // }


// // 通过 + 指定多个 trait bound
// // pub fn pass_more(item: &(impl Summary + Display)){

// // }

// // + 语法也适用于泛型的 trait bound：
// // pub fn pass_more<T: Summary + Display>(item: &T) {

// // }

// // 通过 where 简化 trait bound

// // pub fn some_fun<T: Display + Clone, U: Display + Debug>(item1: &T, item2: &U) -> i32{}
// // where 简化写法
// // pub fn some_fun<T, U>(t: &T, u: &U) -> i32 
// // where 
// //   T: Display + Clone,
// //   U: Display + Debug,
// // {

// // }

// // pub struct Pair<T> {
// //   x: T,
// //   y:T
// // }

// // impl <T> Pair<T> {
// //   fn new(x:T, y: T) -> Self {
// //     Self {x, y}
// //   }
// // }

// // impl <T: Display + PartialOrd> Pair <T> {
// //   fn cmp_display(&self) {
// //     if self.x >= self.y {
// //       println!("The largest value of x: {}", self.x);
// //     } else {
// //       println!("The largest value of y: {}", self.y);
// //     }
// //   }
// // }

// // 这两个参数和返回的引用存活的一样久。（两个）参数和返回的引用的生命周期是相关的
// // 返回结果的作用域，会采用生命周期较短的参数保持一致
// fn longest<'b>(x: &'b str, y: &'b str) -> &'b str {
//   if x.len() > y.len() {
//       x
//   } else {
//       y
//   }
// }

// // 结构体中定义生命周期注解
// struct ImportantExcerpt<'a> {
//   part: &'a str,
// }
// ********** 泛型、Trait、生命周期 END**********

// ********** 编写自动化测试 START**********

// 查看 rust-study/adder/src/lib.rs 文件

// ********** 编写自动化测试  END**********

// ********** 编写一个I/O项目：构建命令行程序  START**********

// 查看 rust/minigrep 文件夹

// ********** 编写一个I/O项目：构建命令行程序  END**********

// ********** Rust 函数式语言功能：迭代器和闭包  START**********
use ferris_says::say;
use std::{thread, time::Duration};
// use art::kinds::PrimaryColor;
// use art::utils::mix;

use art::PrimaryColor;
use art::mix;

/** T恤颜色 */
// 使用#[derive(...)]属性来自动生成Debug, PartialEq, Copy, Clone等trait的实现
// Debug trait 允许我们方便地打印枚举的值
// PartialEq trait 允许我们比较两个枚举值是否相等
// Copy trait 允许我们复制枚举值而不需要额外的堆分配
// Clone trait 是Copy trait的超集，允许我们复制任何类型的值
#[derive(Debug, PartialEq, Copy, Clone)]
enum ShirtColors {
  Red,
  Blue,
}
/** 库存 */
struct  Inventory {
  shirts: Vec<ShirtColors>,
}


impl Inventory {
  // 获取免费颜色的T恤、
  fn giveaway(&self, user_preference: Option<ShirtColors>) -> ShirtColors {
    // || self.most_stocked() 使用了闭包 并在 user_preference 上调用 unwrap_or_else 方法
    // 如果 Option<T> 是 Some 成员，则 unwrap_or_else 返回 Some 中的值。如果 Option<T> 是 None 成员，则 unwrap_or_else 调用闭包并返回闭包的返回值。
    user_preference.unwrap_or_else(|| self.most_stocked())
  }

  // 发放库存最多颜色的T恤
  fn most_stocked(&self) -> ShirtColors {
    // 红色库存数量
    let mut num_red = 0;
    // 蓝色库存数量
    let mut num_blue = 0;
    // 遍历库存、计算数量
    for color in &self.shirts {
      match color {
        ShirtColors::Red => num_red += 1,
        ShirtColors::Blue => num_blue += 1,
      }
    }
    // 红色 > 蓝色 → 返回红色 反之 返回蓝色
    if num_red > num_blue {
        ShirtColors::Red
    } else {
        ShirtColors::Blue
    }
  }
}

fn main() {

    let store = Inventory {
        shirts: vec![ShirtColors::Red, ShirtColors::Blue, ShirtColors::Red,],
    };

    let user_pref1 = Some(ShirtColors::Red);
    let giveaway1 = store.giveaway(user_pref1);
    println!("user_pre1 preference to : {:?} get: {:?}", user_pref1, giveaway1);

    let user_perf2 = None;
    let giveaway2 = store.giveaway(user_perf2);
    println!("user_per1 preference to : {:?} get: {:?}", user_perf2, giveaway2);
    let res = closure(10);
    println!("The res is : {:?} ", res);

    // 捕获引用
    let mut list = vec![1, 2, 3];
    println!("The value of list is: {:#?}", list);

    let only_borrows = || println!("From closure: {:?}", list);

    println!("Before calling closure: {:?}", list);
    only_borrows();
    println!("After calling closure: {:?}", list);

    let mut borrow_mutably = || list.push(7);
    borrow_mutably();
    println!("list push item result: {:?}", list);

    // 在新线程上打印列表 move转移了list 所有权至新线程
    thread::spawn(move || println!("From thread move list: {:?}", list)).join().unwrap();

    // 将捕获的值移出闭包和 Fn trait
    #[derive(Debug)]
    struct Rectangle {
        width: u32,
        #[allow(dead_code)]
        height: u32,
    }

    let mut list2 = [
        Rectangle { width: 10 , height: 7 },
        Rectangle { width: 4 , height: 7 },
        Rectangle { width: 8 , height: 7 },
    ];

    // sort_by_key() 排序
    // list2.sort_by_key(|r| r.width);
    // println!("{:#?}", list2);

    // 统计调用次数
    let mut num_sort_operations = 0;
    list2.sort_by_key(|r| {
        num_sort_operations += 1;
        r.width
    });
    println!("{:#?}, sorted in {num_sort_operations} operations", list2);


    let red = PrimaryColor::Red;
    let yellow = PrimaryColor::Yellow;
    let mix_color = mix(red,yellow);
    println!("The Value of mix is :{:?}", mix_color)
}

// 闭包类型推断和注解

// 定义一个函数，它接受一个闭包作为参数并返回一个数字。
fn closure(num: u32) -> u32 {
    // 定义闭包
    let expensive = |num: u32| -> u32 {
        println!("delay...");
        // 当前线程休眠2s
        thread::sleep(Duration::from_secs(2));
        num
    };
    expensive(num)
}

// ********** Rust 函数式语言功能：迭代器和闭包  END**********
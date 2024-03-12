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

fn main () {
  let s1 = String::from("hello");
  // let s2 = s1.clone();
  // println!("{s2}");
  one(s1);

  let s3 = 10;
  two(s3);
}

fn one (some_string: String) {
  println!("{some_string}")
}
fn two (some_integer: u32) {
  println!("{some_integer}")
}
// ********** 所有权 END************
// use rand::Rng;

// fn main() {
//     let num_rang = rang::thread_rng().gen_range(1..=100);
// }
mod front_of_house;
use crate::front_of_house::hosting;
mod back_of_house;
use crate::back_of_house::redo;

// 前台
// mod front_of_house {
//     // pub：暴露路径，使得不是私有模块
//     pub mod hosting {
//         // 下单
//         pub fn add_to_wait_list() {}
//         // 座位安排
//         #[allow(dead_code)]
//         fn seat_at_table() {}
//     }
//     mod serving {
//         // 接受订单
//         #[allow(dead_code)]
//         fn task_order() {}
//         // 上菜
//         #[allow(dead_code)]
//         fn serve_order() {}
//         // 收款
//         #[allow(dead_code)]
//         fn task_payment() {}
//     }
// }

// 使用 use 创建一个短路径
// pub：使得外部也可导入使用
// pub use crate::front_of_house::hosting;

pub fn eat_at_restaurant() {
    // 报错：module `hosting` is private
    // 原因：在 Rust 中，默认所有项（函数、方法、结构体、枚举、模块和常量）对父模块都是私有的。
    // 绝对路径
    // crate::front_of_house::hosting::add_to_wait_list();
    hosting::add_to_wait_list();

    // 相对路径
    front_of_house::hosting::add_to_wait_list();

    // 重做
    redo::redo_to_wait_list();

    //订购食物: 黑麦
    let mut meal = back_of_house::Breakfast::summer("Rye");
    // 注意改变，订购：面包
    meal.toast = String::from("Wheat");
    println!("I'd like {} toast please", meal.toast);
    
    // 如果取消祖师下一行，代码编译报错
    // 订购水果：蓝莓， 私有变量 无法进行修改会报错
    // meal.seasonal_fruit = String::from("seasonal_fruit")

    // 公有枚举
    let order1 = back_of_house::Appetizer::Soup;
    let order2 = back_of_house::Appetizer::Salad;

    println!("I'd order Appetizer {:?} and {:?}", order1, order2);
}   

// 送货订单
#[allow(dead_code)]
fn deliver_order() {}
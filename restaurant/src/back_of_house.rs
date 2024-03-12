#[allow(dead_code)]
pub fn fix_incorrect_order() {
    cook_order();
    super::deliver_order();
    super::front_of_house::hosting::add_to_wait_list();
}
// 厨师点菜
#[allow(dead_code)]
pub fn cook_order() {}

pub struct Breakfast {
    pub toast: String,
    #[allow(dead_code)]
    seasonal_fruit: String,
}

impl Breakfast {
    pub fn summer(toast: &str) -> Breakfast {
        Breakfast {
            toast: String::from(toast),
            seasonal_fruit: String::from("peaches"),
        }
    }
}

// 开胃小吃
#[derive(Debug)]
pub enum Appetizer {
    Soup,
    Salad,
}

pub mod redo;
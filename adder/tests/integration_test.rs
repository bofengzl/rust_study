use adder;

mod common;

#[test]
fn it_adds_two() {
  common::setup();
  assert_eq!(4, adder::add_two(2))
}


// 指定运行执行集成测试的文件命令： cargo test --test integration_test
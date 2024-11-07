mod basket;
mod stack;
mod container;

use basket::Basket;
use stack::Stack;
use container::add_string;

fn main() {
    let mut b1 = Basket::new(42);
    let mut b2 = Basket::new(String::from("hello"));
    let mut b3 = Basket::new(3.14);
    let mut b4 = Basket::new(vec![1, 2, 3]);
    
    let mut s1 = Stack::new(vec![1, 2, 3]);
    let mut s2 = Stack::new(vec!["hello", "world"]);
    let mut s3 = Stack::new(vec![3.14, 2.71]);

    add_string(&mut b2, "world".to_string());
}

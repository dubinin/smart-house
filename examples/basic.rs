//! Пример использование библиотеки для реализации умного дома.

use smart_house::house::SmartHouse;

fn main() {
    let house: SmartHouse = Default::default();

    println!("{}", house.name());
}

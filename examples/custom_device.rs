//! Пример использование библиотеки умного дома с пользовательскими устройствами.

use smart_house::device::{Device, DisplayableDevice};
use smart_house::house::{Room, SmartHouse};

struct MyDevice {}

impl Device for MyDevice {
    fn name(&self) -> &str {
        "Пользовательское"
    }

    fn on(&mut self) -> bool {
        true
    }

    fn off(&mut self) -> bool {
        false
    }

    fn power(&self) -> u16 {
        16
    }
}

impl std::fmt::Display for MyDevice {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Я простое устройство по имени: {}", self.name())
    }
}

impl DisplayableDevice for MyDevice {}

fn main() {
    // Создаем умный дом с название по умолчанию.
    let mut house: SmartHouse = Default::default();

    // Создаем первую комнату умного дома.
    let mut room = Room::with_name("Первая комната");

    // Создаем пользовательское устройство.
    let device = MyDevice {};

    // Добавляем устройство в комнату.
    room.attach_device(&device).unwrap();

    // Добавляем комнату к дому.
    house.attach_room(room).unwrap();

    // Формируем отчет по дому.
    println!("{}", house.report());
}

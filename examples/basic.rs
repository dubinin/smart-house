//! Пример использование библиотеки для реализации умного дома.

use smart_house::device::{SmartSocket, SmartThermometer};
use smart_house::house::{Room, SmartHouse};

fn main() {
    // Создаем умный дом с название по умолчанию.
    let mut house: SmartHouse = Default::default();

    // Создаем устройства умного дома.
    let mut socket_first = SmartSocket::with_name("розетка №1");
    let socket_second = SmartSocket::with_name("розетка №2");
    let thermometer_first = SmartThermometer::with_name("термометр №1");
    let thermometer_second = SmartThermometer::with_name("термометр №2");

    // Добавляем к умной розетки два устройства.
    socket_first.attach_device(&thermometer_first).unwrap();
    socket_first.attach_device(&thermometer_second).unwrap();

    // Создаем первую комнату умного дома.
    let mut kitchen = Room::with_name("Кухня");

    // Добавляем устройства к первой комнате.
    kitchen.attach_device(&socket_first).unwrap();
    kitchen.attach_device(&thermometer_first).unwrap();
    kitchen.attach_device(&thermometer_second).unwrap();

    // Добавляем первую комнату к дому.
    house.attach_room(kitchen).unwrap();

    // Создаем вторую комнату.
    let mut bedroom = Room::with_name("Спальня");

    // Добавляем устройство к второй комнате.
    bedroom.attach_device(&socket_second).unwrap();

    // Добавляем вторую комнату к дому.
    house.attach_room(bedroom).unwrap();

    // Вывод отчета.
    println!("{}", house.report());
}

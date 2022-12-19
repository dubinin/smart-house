//! Модуль для реализация умной разетки.

use super::Device;

/// Структура умной разетки, которая хранит ссылки на устройства подключенные к ней.
pub struct SmartSocket<'a> {
    name: &'a str,
    devices: Vec<&'a dyn Device>,
}

impl <'a>Device for SmartSocket<'a> {
    fn name(&self) -> &str {
        self.name
    }

    fn power(&self) -> u16 {
        todo!("Реализовать метод")
    }

    fn report(&self) -> String {
        todo!("Реализовать метод")
    }
}
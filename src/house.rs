//! Модуль содержащий реализацию сущности умного дома.

use std::collections::HashMap;
use super::device::Device;

/// Структура умного дома, содержит название и имеет список комнат.
pub struct SmartHouse<'a> {
    name: &'a str,
    rooms: HashMap<String, Room<'a>>,
}

/// Конструктор по умолчанию для умного дома.
impl<'a> Default for SmartHouse<'a> {
    fn default() -> Self {
        SmartHouse::with_name("Умный дом")
    }
}

/// Реализация функций умного дома.
impl<'a> SmartHouse<'a> {
    /// Конструктуро умного дома с переданным названием.
    pub fn with_name(name: &'a str) -> Self {
        Self { name, rooms: HashMap::new() }
    }

    /// Получить название умного дома.
    pub fn name(&self) -> &str {
        self.name
    }

    /// Добавить комнату к умному дому.
    pub fn attach_room(&mut self, room: Room<'a>) {
        self.rooms.insert(room.name(), room);
    }
}

/// Структура реализующая комнату в умном доме, содержит название и список устройств.
pub struct Room<'a> {
    name: &'a str,
    devices: Vec<&'a dyn Device>,
}

/// Реализация функций комнаты умного дома.
impl<'a> Room<'a> {
    /// Получить название комнаты.
    fn name(&self) -> String {
        self.name.to_string()
    }
}

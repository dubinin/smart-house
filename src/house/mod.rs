//! Модуль содержащий реализацию сущности умного дома.

mod room;

use crate::error::{attachment_error, AttachmentError};
pub use crate::house::room::Room;

use std::collections::HashMap;

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
        Self {
            name,
            rooms: Default::default(),
        }
    }

    /// Получить название умного дома.
    pub fn name(&self) -> &str {
        self.name
    }

    /// Добавить комнату к умному дому.
    pub fn attach_room(&mut self, room: Room<'a>) -> Result<(), AttachmentError> {
        let is_exist = |name: &str| -> bool { self.rooms.contains_key(name) };

        attachment_error(room.name(), is_exist).map(|_| {
            self.rooms.insert(room.name().to_string(), room);
        })
    }

    /// Формирование отчета для умного дома.
    pub fn report(&self) -> String {
        let mut result = String::new();

        result.push_str(&format!("Отчет для дома: {}\n", self.name));

        for room in self.rooms.values() {
            result.push_str(&format!("{:-<20}\n", ""));
            result.push_str(&format!("Комната: {}\n", room.name()));

            for device in room.devices.values() {
                result.push_str(&format!("\t{}\n", device));
            }
        }

        result
    }
}

#[cfg(test)]
mod test;

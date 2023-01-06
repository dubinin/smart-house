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
mod tests {
    use super::*;

    /// Можно создать дом переда имя для него.
    #[test]
    fn test_create_house_with_name() {
        let house = SmartHouse::with_name("Тестовый дом");

        assert_eq!(house.name(), "Тестовый дом");
    }

    /// Можно добавлять комнаты к умному дому.
    #[test]
    fn test_attach_room_to_house() {
        let mut house = SmartHouse::default();
        let room = Room::with_name("Комната 1");
        let attach_result = house.attach_room(room);

        assert!(attach_result.is_ok());
        assert_eq!(house.rooms.len(), 1);
    }

    /// Если к дому добавляется комната с слишком длинным названием это ошибка.
    #[test]
    fn test_attach_with_long_name() {
        let mut house = SmartHouse::default();
        let room = Room::with_name("Неприлично длинное название для комнаты!");
        let attach_result = house.attach_room(room);

        assert!(attach_result.is_err());
        assert!(matches!(
            attach_result.unwrap_err(),
            AttachmentError::LongName
        ));
    }

    /// При добавлении комнаты к дому с слишком коротким названием будет ошибка.
    #[test]
    fn test_attach_with_short_name() {
        let mut house = SmartHouse::default();
        let room = Room::with_name("К");
        let attach_result = house.attach_room(room);

        assert!(attach_result.is_err());
        assert!(matches!(
            attach_result.unwrap_err(),
            AttachmentError::ShortName
        ));
    }

    /// Дом может сформировать отчет.
    #[test]
    fn test_smart_house_report() {
        let mut house = SmartHouse::default();
        let room = Room::with_name("Комната");
        house.attach_room(room).unwrap();
        let report = house.report();

        assert!(!report.is_empty());
    }

    /// Создание комнаты с указанием ее имени.
    #[test]
    fn test_create_room_with_name() {
        let room = Room::with_name("Комната");

        assert_eq!(room.name(), "Комната");
    }
}

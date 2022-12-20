//! Модуль содержащий реализацию сущности умного дома.

use crate::error::{attachment_error, AttachmentError};

use super::device::Device;
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

        attachment_error(room.name, is_exist).map(|_| {
            self.rooms.insert(room.name(), room);
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
                // Почему не удается просто передавать device? Я сделал реализацию трейта
                // Display для Device, соответственно device.to_string() тоже не работает.
                // Не могу понять в чем принципиальное отличие от рабочего варианта.
                // result.push_str(&format!("\t{}\n", device));
                result.push_str(&format!("\t{}\n", device.report()));
            }
        }

        result
    }
}

/// Структура реализующая комнату в умном доме, содержит название и список устройств.
pub struct Room<'a> {
    name: &'a str,
    devices: HashMap<&'a str, &'a dyn Device>,
}

/// Реализация функций комнаты умного дома.
impl<'a> Room<'a> {
    /// Конструктуро комнаты с переданным названием.
    pub fn with_name(name: &'a str) -> Self {
        Self {
            name,
            devices: Default::default(),
        }
    }

    /// Получить название комнаты.
    pub fn name(&self) -> String {
        self.name.to_string()
    }

    /// Функция добавление устройства в комнату.
    pub fn attach_device(&mut self, device: &'a dyn Device) -> Result<(), AttachmentError> {
        let is_exist = |name: &str| -> bool { self.devices.contains_key(name) };
        let device_name = device.name();

        attachment_error(device_name, is_exist).map(|_| {
            self.devices.insert(device_name, device);
        })
    }
}

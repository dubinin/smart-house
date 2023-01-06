//! Модуль реализации комнаты умного дома.

use crate::device::DisplayableDevice;
use crate::error::{attachment_error, AttachmentError};

use std::collections::HashMap;

/// Структура реализующая комнату в умном доме, содержит название и список устройств.
pub struct Room<'a> {
    name: &'a str,
    pub devices: HashMap<String, &'a dyn DisplayableDevice>,
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
    pub fn name(&self) -> &str {
        self.name
    }

    /// Функция добавление устройства в комнату.
    pub fn attach_device(
        &mut self,
        device: &'a dyn DisplayableDevice,
    ) -> Result<(), AttachmentError> {
        let is_exist = |name: &str| -> bool { self.devices.contains_key(name) };
        let device_name = device.name();

        attachment_error(device_name, is_exist).map(|_| {
            self.devices.insert(device_name.to_string(), device);
        })
    }
}

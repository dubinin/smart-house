//! Модуль для реализация умной розетки.

use std::collections::HashMap;

use crate::error::{attachment_error, AttachmentError};

use super::{Device, DisplayableDevice};

/// Структура умной розетки, которая хранит ссылки на устройства подключенные к ней.
pub struct SmartSocket<'a> {
    name: &'a str,
    devices: HashMap<&'a str, &'a dyn Device>,
}

/// Ошибки возникающие при использовании розетки.
#[derive(Debug)]
pub enum SocketError {
    AttachmentError(AttachmentError),
    WrongPower,
}

impl std::fmt::Display for SocketError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let message = match self {
            Self::AttachmentError(e) => e.to_string(),
            Self::WrongPower => String::from(
                "Нельзя подключать в розетку устройство которое не потребляет энергию!",
            ),
        };
        write!(f, "{}", message)
    }
}

impl std::error::Error for SocketError {}

/// Реализация функционала устройства для умной розетки.
impl<'a> Device for SmartSocket<'a> {
    /// Получить имя умной розетки.
    fn name(&self) -> &str {
        self.name
    }

    /// Значение потребляемой мощности. Есть сумма потребляемых мощностей подключенных
    /// к розетки других устройств. У самой розетки потребляемая мощность равно 0.
    fn power(&self) -> u16 {
        self.devices.iter().map(|device| device.1.power()).sum()
    }
}

/// Трейт Display используется для составления отчета по розетки.
impl<'a> std::fmt::Display for SmartSocket<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Умная розетка: {}, подключено устройств: {}, потребляемая мощность: {}",
            self.name(),
            self.devices.len(),
            self.power()
        )
    }
}

impl<'a> DisplayableDevice for SmartSocket<'a> {}

/// Функциональность специфичная для умной розетки.
impl<'a> SmartSocket<'a> {
    /// Конструктор розетки с передачей ее названия.
    pub fn with_name(name: &'a str) -> Self {
        Self {
            name,
            devices: Default::default(),
        }
    }

    /// Функция подключения устройства к розетке.
    pub fn attach_device(&mut self, device: &'a dyn Device) -> Result<u16, SocketError> {
        //! Если потребляемая мощность устройства равно 0, то считаем это ошибкой.
        if device.power() == 0 {
            return Err(SocketError::WrongPower);
        }

        let is_exist = |name: &str| -> bool { self.devices.contains_key(name) };
        let device_name = device.name();

        match attachment_error(device_name, is_exist) {
            Ok(_) => {
                self.devices.insert(device_name, device);
                Ok(self.power())
            }
            Err(e) => Err(SocketError::AttachmentError(e)),
        }
    }
}

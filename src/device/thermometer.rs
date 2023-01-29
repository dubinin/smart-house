//! Модуль для реализация умного термометра.

use super::{Device, DisplayableDevice};

/// Структура умного термометра.
/// Данные по температуре не хранятся в самой структуре. А получаются по факту вызова
/// соответсвующего метода.
pub struct SmartThermometer<'a> {
    name: &'a str,
    temperature: f32,
}

/// Шкалы температур для котрых можно получить данные от умного термометра.
pub enum TemperatureScale {
    Kelvin,
    Celsius,
    Fahrenheit,
}

/// Функции для шкалы температур
impl TemperatureScale {
    /// Преобразование значения температуры для разных температурных шкал.
    /// Предполагается что на вход передается значение в Цельсиях.
    fn transform(&self, value: f32) -> f32 {
        match *self {
            Self::Celsius => value,
            Self::Fahrenheit => (value * (9.0 / 5.0)) + 32.0,
            Self::Kelvin => value + 273.15,
        }
    }
}

/// Символы для отображения температурной шкалы.
impl std::fmt::Display for TemperatureScale {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let description = match *self {
            Self::Celsius => "°C",
            Self::Fahrenheit => "°F",
            Self::Kelvin => "K",
        };
        write!(f, "{}", description)
    }
}

/// Реализация функционала устройства для умного термометра.
impl<'a> Device for SmartThermometer<'a> {
    /// Получить имя термометра.
    fn name(&self) -> &str {
        self.name
    }

    /// Термометр всегда вкключен.
    fn on(&mut self) -> bool {
        true
    }

    /// Термометр нельзя выключить.
    fn off(&mut self) -> bool {
        false
    }

    /// Константное значение потребляемой мощности.
    fn power(&self) -> u16 {
        20
    }
}

/// Трейт Display используется для составления отчета для термометра.
impl<'a> std::fmt::Display for SmartThermometer<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let scale = TemperatureScale::Celsius;
        write!(
            f,
            "Умный термометр: {}, температура: {} {}, потребляемая мощность: {}",
            self.name(),
            self.temperature(&scale),
            scale,
            self.power()
        )
    }
}

impl<'a> DisplayableDevice for SmartThermometer<'a> {}

/// Реализация функций специфичных для умного термометра.
impl<'a> SmartThermometer<'a> {
    /// Конструктор термометра с передачей ее названия.
    pub fn with_name(name: &'a str) -> Self {
        Self {
            name,
            temperature: 0.0,
        }
    }

    /// Функция получения значения температуры в зависимости от переданной температурной шкалы.
    pub fn temperature(&self, scale: &TemperatureScale) -> f32 {
        scale.transform(self.temperature)
    }

    pub fn set_temperature(&mut self, value: f32) {
        self.temperature = value;
    }
}

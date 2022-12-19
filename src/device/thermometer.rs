//! Модуль для реализация умного термометра.

use super::Device;
use rand::Rng;

/// Структура умного термометра.
/// Данные по температуре не хранятся в самой структуре. А получаются по факту вызова
/// соответсвующего метода.
pub struct SmartThermometer<'a> {
    name: &'a str,
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

/// Реализация функционала устройства для умного термометра.
impl <'a>Device for SmartThermometer<'a> {
    fn name(&self) -> &str {
        self.name
    }

    /// Константное значение потребляемой мощности.
    fn power(&self) -> u16 {
        20
    }

    /// Функция получения отчета устройства.
    fn report(&self) -> String {
        todo!("Реализовать метод")
    }
}

/// Реализация функций специфичных для умного термометра.
impl<'a> SmartThermometer<'a> {
    /// Функция получения значения температуры в зависимости от переданной температурной шкалы.
    pub fn temperature(&self, scale: TemperatureScale) -> f32 {
        let mut rng = rand::thread_rng();
        let raw_temperature = rng.gen_range(-20.0..=40.0);
        scale.transform(raw_temperature)
    }
}

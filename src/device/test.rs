//! Модуль с тестами для устройств библиотеки умный дом.

use super::thermometer::TemperatureScale;
use crate::device::socket::SocketError;

use super::*;

/// Можно создать устройство и общие для всех устройств функции работают.
#[test]
fn test_device_fields() {
    let device = SmartSocket::with_name("Розетка");

    assert_eq!(device.name(), "Розетка");
    assert_eq!(device.power(), 0u16);
    assert!(!device.report().is_empty());
}

/// Проверим что к розетки нельзя добавить устройство которое не потребляет электроэнергию.
#[test]
fn test_unable_to_attach_zero_power_device_to_socket() {
    let mut socket = SmartSocket::with_name("Розетка");
    let zero_power_device = SmartSocket::with_name("Еще розетка");
    let attach_result = socket.attach_device(&zero_power_device);

    assert!(matches!(
        attach_result.unwrap_err(),
        SocketError::WrongPower
    ));
}

/// Проверим что термометр без проблем возвращает температуру по разным температурным шкалам.
/// Сейчас нет возможности для теста указать статическое значение которое будет возвращать функция
/// temperature, поэтому просто проверим что функции отрабатывают с разными входными параметрами.
/// В будущем можно сделать mock который бы для теста возвращал всегда статичное значение температуры
/// и проверять преобразование этой температуры для разных шкал.
#[test]
fn test_thermometer_return_temperature() {
    let thermometer = SmartThermometer::with_name("Термометр");

    thermometer.temperature(&TemperatureScale::Kelvin);
    thermometer.temperature(&TemperatureScale::Celsius);
    thermometer.temperature(&TemperatureScale::Fahrenheit);
}

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

/// Итератор по дому обходит комнаты.
#[test]
fn test_house_iterator_over_rooms() {
    let mut house = SmartHouse::default();
    let room = Room::with_name("Комната");
    house.attach_room(room).unwrap();

    let rooms: Vec<&str> = house.into_iter().map(|r| -> &str { r.name() }).collect();

    assert_eq!(rooms.len(), 1);
    assert_eq!(rooms[0], "Комната");
}

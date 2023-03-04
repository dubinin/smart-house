//! Модуль содержащий ошибки возникающие при работе библиотеки умный дом.
use thiserror::Error;

/// Ошибки прикрепления элемента к различным сущностям.
#[derive(Error, Debug)]
pub enum AttachmentError {
    #[error("Запись с таким именем уже была добавлена!")]
    AlreadyExist,
    #[error("Переданное имя слишком короткое!")]
    ShortName,
    #[error("Переданное имя слишком длинное!")]
    LongName,
}

pub fn attachment_error<P>(name: &str, is_exist: P) -> Result<(), AttachmentError>
where
    P: Fn(&str) -> bool,
{
    if is_exist(name) {
        return Err(AttachmentError::AlreadyExist);
    }
    match name.chars().count() {
        0..=3 => Err(AttachmentError::ShortName),
        20.. => Err(AttachmentError::LongName),
        _ => Ok(()),
    }
}

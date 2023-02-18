//! Модуль содержащий ошибки возникающие при работе библиотеки умный дом.

/// Ошибки прикрепления элемента к различным сущностям.
#[derive(Debug)]
pub enum AttachmentError {
    AlreadyExist,
    ShortName,
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

impl std::fmt::Display for AttachmentError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let message = match self {
            Self::AlreadyExist => "Запись с таким именем уже была добавлена!",
            Self::ShortName => "Переданное имя слишком короткое!",
            Self::LongName => "Переданное имя слишком длинное!",
        };
        write!(f, "{message}")
    }
}

impl std::error::Error for AttachmentError {}

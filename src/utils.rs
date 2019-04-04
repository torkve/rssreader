use std::fmt::Display;

pub(crate) trait ErrString<T> {
    fn or_err(self, error_message: &str) -> Result<T, String>;
}

impl<T, E: Display> ErrString<T> for Result<T, E> {
    fn or_err(self, error_message: &str) -> Result<T, String> {
        self.map_err(|e| format!("{}: {}", error_message, e))
    }
}

impl<T> ErrString<T> for Option<T> {
    fn or_err(self, error_message: &str) -> Result<T, String> {
        self.ok_or(error_message.to_string())
    }
}

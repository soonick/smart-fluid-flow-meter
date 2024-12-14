pub enum ErrorCode {
    UndefinedError,
    DuplicateError,
}

pub struct Error {
    pub code: ErrorCode,
}

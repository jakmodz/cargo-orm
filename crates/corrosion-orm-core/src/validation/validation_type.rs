pub enum ValidationType {
    NotNull,
    Size {
        min: Option<usize>,
        max: Option<usize>,
    },
    Regex {
        pattern: String,
    },
    Email,
}

pub struct Validation {
    pub message: String,
    pub ty: ValidationType,
}

impl Validation {
    pub fn new(ty: ValidationType, msg: String) -> Self {
        Self { message: msg, ty }
    }
}

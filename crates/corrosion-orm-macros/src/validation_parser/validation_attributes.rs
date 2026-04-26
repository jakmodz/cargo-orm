use deluxe::ExtractAttributes;

#[derive(ExtractAttributes, Debug)]
#[deluxe(attributes(NotNull))]
pub struct NotNullAttribute {
    #[deluxe(default = None)]
    pub message: Option<String>,
}

#[derive(ExtractAttributes, Debug)]
#[deluxe(attributes(Size))]
pub struct SizeAttribute {
    #[deluxe(default = None)]
    pub message: Option<String>,
    #[deluxe(default = None)]
    pub min: Option<usize>,
    #[deluxe(default = None)]
    pub max: Option<usize>,
}

#[derive(ExtractAttributes, Debug)]
#[deluxe(attributes(Pattern))]
pub struct PatternAttribute {
    #[deluxe(default = None)]
    pub message: Option<String>,
    pub pattern: String,
}

#[derive(ExtractAttributes, Debug)]
#[deluxe(attributes(Email))]
pub struct EmailAttribute {
    #[deluxe(default = None)]
    pub message: Option<String>,
}

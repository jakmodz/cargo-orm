pub trait ColumnTrait: Copy + Send + Sync {
    fn as_str(&self) -> &'static str;
}

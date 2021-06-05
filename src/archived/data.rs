#[derive(Copy, Clone, Debug, PartialEq)]
pub enum Type {
    Empty,
    String,
    Color,
    Image,
    Pixel,
    U32,
}

#[derive(Clone, Debug, PartialEq)]
pub enum Data {
    String(String),
    U32(u32),
}
impl Data {
    pub fn get_type(&self) -> Type {
        todo!();
    }
}

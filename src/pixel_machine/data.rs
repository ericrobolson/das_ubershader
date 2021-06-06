#[derive(Debug)]
pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub a: u8,
}

#[derive(Debug)]
pub enum Type {
    U32,
    String,
    Pixel,
    Color,
}

#[derive(Debug)]
pub enum Data {
    Bool(bool),
    Color(Color),
    Pixel(Pixel),
    U32(u32),
}
impl Data {
    pub fn get_type(&self) -> Type {
        match self {
            Data::Bool(_) => todo!(),
            Data::Color(_) => todo!(),
            Data::Pixel(_) => todo!(),
            Data::U32(_) => Type::U32,
        }
    }
}
#[derive(Debug)]
pub struct Pixel {
    pub x: u32,
    pub y: u32,
}

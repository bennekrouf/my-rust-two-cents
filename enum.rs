use int_enum::IntEnum;
#[derive(Debug, PartialEq, IntEnum)]
pub enum ResistorColor {
    Black,
    Blue,
    Brown,
    Green,
    Grey,
    Orange,
    Red,
    Violet,
    White,
    Yellow,
}

pub fn color_to_value(_color: ResistorColor) -> usize {
    3
}

pub fn main() {
    let b = color_to_value(ResistorColor::Black);
    println!("{:?}", b);
}
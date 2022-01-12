use int_enum::IntEnum;

#[repr(u8)]
#[derive(Debug, PartialEq, Clone, Copy, IntEnum)]
pub enum ResistorColor {
    Black = 0,
    Blue = 6,
    Brown = 1,
    Green = 5,
    Grey = 8,
    Orange = 3,
    Red = 2,
    Violet = 7,
    White = 9,
    Yellow = 4,
}

// pub fn color_to_value(_color: ResistorColor) -> usize {
//     _color.int_value().into()
// }

pub fn value_to_color_string(value: usize) -> String {
    let a:[ResistorColor; 10] = [
        ResistorColor::Black,
        ResistorColor::Brown,
        ResistorColor::Red,
        ResistorColor::Orange,
        ResistorColor::Yellow,
        ResistorColor::Green,
        ResistorColor::Blue,
        ResistorColor::Violet,
        ResistorColor::Grey,
        ResistorColor::White,
    ];
    a[value].to_string()
}

// pub fn printme(value: ResistorColor) {
//     let color = color_to_value(value);
//     println!("{:?}", color);
// }

pub fn main() {
    // let b = color_to_value(ResistorColor::Black);
    // println!("{:?}", b);

    // let tt = ResistorColor::from_int(3).unwrap();
    // printme(tt);
    // // println!("{:?}", tt);
    let a:[ResistorColor; 10] = [
        ResistorColor::Black,
        ResistorColor::Brown,
        ResistorColor::Red,
        ResistorColor::Orange,
        ResistorColor::Yellow,
        ResistorColor::Green,
        ResistorColor::Blue,
        ResistorColor::Violet,
        ResistorColor::Grey,
        ResistorColor::White,
    ];

    // let c = value_to_color_string(3);
    println!("{:?}", a[0]);
}
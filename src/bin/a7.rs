// Topic: Working with an enum
//
// Program requirements:
// * Prints the name of a color to the terminal
//
// Notes:
// * Use an enum with color names as variants
// * Use a function to print the color name
// * The function must use the enum as a parameter
// * Use a match expression to determine which color
//   name to print

enum Color {
    Red,
    Blue,
    Black,
    White,
}

fn color_name(color:Color) {
    match color {
        Color::Red => print!("The color is red!"),
        Color::Blue => print!("The color is blue!"),
        Color::Black => print!("The color is black!"),
        Color::White => print!("The color is white!"),
    }
}

fn main() {

    color_name(Color::Red);
}

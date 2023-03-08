// `enum`s  define a type that can only take on one of several specified values.
// `Message` is an enum type that has four variants: Quit, Echo, Move, and ChangeColor
#[derive(Debug)]
enum Message {
    Quit,
    Echo,
    Move,
    ChangeColor,
}

fn main() {
    println!("{:?}", Message::Quit);
    println!("{:?}", Message::Echo);
    println!("{:?}", Message::Move);
    println!("{:?}", Message::ChangeColor);
}

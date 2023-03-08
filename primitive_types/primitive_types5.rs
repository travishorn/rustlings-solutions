fn main() {
    let cat = ("Furry McFurson", 3.5);

    // You can destructure tuples. The variables `name` and `age` will reference the first and
    // second elements (respectively) in the cat tuple
    let (name, age) = cat;

    println!("{} is {} years old.", name, age);
}

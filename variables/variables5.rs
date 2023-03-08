fn main() {
    let number = "T-H-R-E-E";
    println!("Spell a Number : {}", number);

    // Use "shadowing" to declare a new variable using the same name as a previous variable.
    // Shadow a variable by using `let` again
    let number = 3;
    println!("Number plus two is : {}", number + 2);
}

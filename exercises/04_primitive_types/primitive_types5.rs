fn main() {
    let cat = ("Furry McFurson", 3.5);

    // TODO: Destructure the `cat` tuple in one statement so that the println works.
    // match cat {
    //     (name, age) => println!("{name} is {age} years old"),
    // }
    let (name, age): (&str, f64) = cat;
    println!("{name} is {age} years old")
}

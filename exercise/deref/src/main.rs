fn printme(s: &String) {
    println!("Value is {}", *s);
}

fn main() {
    let a = String::from("lorem");
    printme(&a);
    println!("I has {}", a);
}

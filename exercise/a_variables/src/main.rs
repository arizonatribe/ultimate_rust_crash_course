const STARTING_MISSILES: i32 = 8;
const READY_AMOUNT: i32 = 2;

fn main() {
    let (mut missles, ready) = (STARTING_MISSILES, READY_AMOUNT);
    println!("Firing {} of my {} missles ...", ready, missles);
    missles = missles - 1;
    println!("{} missles left", missles);
}

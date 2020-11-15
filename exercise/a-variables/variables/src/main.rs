

fn main() {
    const STARTING_MISSILES: i32, READY_AMOUNT: i32 = {8 ,2};

    let mut missiles: i32 = STARTING_MISSILES;
    let ready: i32 = READY_AMOUNT;
    println!("Firing {} of my {} missiles...", ready, missiles);

    missiles = missiles - ready;
    println!("We have {} missiles left", missiles)
}

use rand::Rng;
use std::io::{self};

fn main() -> io::Result<()> {
    let n: i32 = std::env::args().nth(1).expect("Please provide number of IP addresses").parse().expect("Please provide a valid number");
    
    for _ in 0..n {
        let ip_address = generate_random_ip();
        println!("{}", ip_address);
    }
    Ok(())
}

fn generate_random_ip() -> String {
    let mut rng = rand::thread_rng();
    let octets: Vec<u8> = (0..4).map(|_| rng.gen_range(0..=255)).collect();
    format!("{}.{}.{}.{}", octets[0], octets[1], octets[2], octets[3])
}

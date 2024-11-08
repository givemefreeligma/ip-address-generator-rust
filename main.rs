use rand::Rng;
use std::fs::File;
use std::io::{self, Write};

fn main() -> io::Result<()> {
    let mut file = File::create("ip_addresses.txt")?;
    
    for _ in 0..10000000000000000000000000000000000_i128 {
        let ip_address = generate_random_ip();
        writeln!(file, "{}", ip_address)?;
    }
    
    println!("100 IP addresses have been written to ip_addresses.txt");
    Ok(())
}

fn generate_random_ip() -> String {
    let mut rng = rand::thread_rng();
    let octets: Vec<u8> = (0..4).map(|_| rng.gen_range(0..=255)).collect();
    format!("{}.{}.{}.{}", octets[0], octets[1], octets[2], octets[3])
}

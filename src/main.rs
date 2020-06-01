use std::io;

fn main() {
    println!("Let's calculate some Voltage Divider Tensions !");
    let mut vin = String::new();
    let mut r1 = String::new();
    let mut r2 = String::new();
    let mut _vout: f64;
    println!("Vin:");
    io::stdin()
        .read_line(&mut vin)
        .expect("Failed to read line");
    println!("
R1:");
    io::stdin()
        .read_line(&mut r1)
        .expect("Failed to read line");
    println!("
R2:");
    io::stdin()
        .read_line(&mut r2)
        .expect("Failed to read line");
    let vin: f64 = vin.trim().parse().expect("Please type a number!");
    let r1: f64 = r1.trim().parse().expect("Please type a number!");
    let r2: f64 = r2.trim().parse().expect("Please type a number!");
    let _first: f64 = r1 + r2;
    let _second: f64 = r2 / _first;
    let _vout = vin * _second;
    //Brut formula is ``` vout = vin * ( r2 / ( r1 + r2 ) ) ```
    println!("With Vin {} V, R1 = {} Ohms, R2 = {} Ohms,", vin, r1, r2);
    println!("Vout would be {:.2} Volts", _vout);
}

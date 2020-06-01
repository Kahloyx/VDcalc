use std::io;

fn main() {
    voltage_divider();
    current_divider();
}
fn voltage_divider(){
    println!("Let's calculate some Voltage Divider Tensions !");
    let mut _vin = String::new();
    let mut r1 = String::new();
    let mut r2 = String::new();
    let mut _vout: f64;
    println!("Vin in V:");
    io::stdin()
        .read_line(&mut _vin)
        .expect("Failed to read line");
    println!("
R1 in Ohms:");
    io::stdin()
        .read_line(&mut r1)
        .expect("Failed to read line");
    println!("
R2 in Ohms:");
    io::stdin()
        .read_line(&mut r2)
        .expect("Failed to read line");
    let _vin: f64 = _vin.trim().parse().expect("Please type a number!");
    let r1: f64 = r1.trim().parse().expect("Please type a number!");
    let r2: f64 = r2.trim().parse().expect("Please type a number!");
    let _first: f64 = r1 + r2;
    let _second: f64 = r2 / _first;
    let _vout = _vin * _second;
    //Brut formula is ``` vout = vin * ( r2 / ( r1 + r2 ) ) ```
    println!("With Vin {} V, R1 = {} Ohms, R2 = {} Ohms,", _vin, r1, r2);
    println!("Vout would be {:.2} Volts", _vout);
}

fn current_divider(){
    println!("Let's calculate some Current Divider Amps !");
    let mut _iin = String::new();
    let mut r1 = String::new();
    let mut r2 = String::new();
    let mut _iout: f64;
    println!("Iin in mA:");
    io::stdin()
        .read_line(&mut _iin)
        .expect("Failed to read line");
    println!("
R1 in Ohms:");
    io::stdin()
        .read_line(&mut r1)
        .expect("Failed to read line");
    println!("
R2 in Ohms:");
    io::stdin()
        .read_line(&mut r2)
        .expect("Failed to read line");
    let _iin: f64 = _iin.trim().parse().expect("Please type a number!");
    let r1: f64 = r1.trim().parse().expect("Please type a number!");
    let r2: f64 = r2.trim().parse().expect("Please type a number!");
    let _first: f64 = r1 + r2;
    let _second: f64 = r1 / _first;
    let _iout = _iin * _second;
    //Brut formula is ``` Iout = Iin * ( r1 / ( r1 + r2 ) ) ```
    println!("With Iin {} mA, R1 = {} Ohms, R2 = {} Ohms,", _iin, r1, r2);
    println!("Vout would be {:.2} mA", _iout);
}

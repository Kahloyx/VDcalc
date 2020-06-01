use std::io;

fn main() {
    loop {
        let mut volts_in = String::new();
        let mut _outvolts:f64;
        let mut _out_amps:f64;
        let mut amps_in = String::new();
        let mut r1 = String::new();
        let mut r2 = String::new();
        println!("Vin in V:");
        io::stdin().read_line(&mut volts_in).expect("Failed to read line");
        let volts_in:f64 = match volts_in.trim().parse(){
            Ok(num) => num,
            Err(_) => continue,
        };
        println!("
Iin in mA:");
        io::stdin().read_line(&mut amps_in).expect("Failed to read line");
        let amps_in:f64 = match amps_in.trim().parse(){
            Ok(num) => num,
            Err(_) => continue,
        };
        println!("
R1 in Ohms:");
        io::stdin().read_line(&mut r1).expect("Failed to read line");
        let r1:f64 = match r1.trim().parse(){
            Ok(num) => num,
            Err(_) => continue,
        };
        println!("
R2 in Ohms:");
        io::stdin().read_line(&mut r2).expect("Failed to read line");
        let r2:f64 = match r2.trim().parse(){
            Ok(num) => num,
            Err(_) => continue,
        };
        let _outvolts = voltage_divider(volts_in, r1, r2);
        let _out_amps = current_divider(amps_in, r1, r2);
        println!("
With Vin:{} V, Iin:{} mA, R1:{} Ohms, R2:{} Ohms, we obtain Vout:{:.3} V and Iout:{:.3} mA", volts_in, amps_in, r1, r2, _outvolts, _out_amps);
        break;
    }
}

fn voltage_divider(volts_in:f64, r1:f64, r2:f64) -> f64 {
    let _first: f64 = r1 + r2;
    let _second: f64 = r2 / _first;
    let mut _outvolts:f64 = volts_in * _second;                                         //Brut formula is ``` vout = vin * ( r2 / ( r1 + r2 ) ) ```
    return _outvolts;
}

fn current_divider(amps_in:f64, r1:f64, r2:f64) -> f64{
    let _first: f64 = r1 + r2;
    let _second: f64 = r1 / _first;
    let mut _out_amps:f64 = amps_in * _second;                                          //Brut formula is ``` Iout = Iin * ( r1 / ( r1 + r2 ) ) ```
    return _out_amps;
}

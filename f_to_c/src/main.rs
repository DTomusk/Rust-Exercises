use std::io;

fn f_to_c(input: String) -> Result<f32, std::num::ParseFloatError> {
    let input = input.trim();
    let temp: f32 = input.parse()?;
    Ok((temp - 32.0) * (5.0 / 9.0))
}

fn main() {
    println!("Input temperature in Fahrenheit");
    let mut temperature = String::new();
    io::stdin().read_line(&mut temperature).unwrap();
    match f_to_c(temperature) {
        Ok(a) => println!("Celsius is {}", a),
        Err(_) => panic!("Unable to convert to celsius"),
    }
}

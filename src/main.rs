use std::io;

fn f_to_c(f: f32) -> f32 {
    (f - 32.0) / 1.8
}

fn c_to_f(c: f32) -> f32 {
    (c * 1.8) + 32.0
}

fn main() {
    let mut temp_value = String::new();
    let mut temp_unit = String::new();

    loop {
        println!("Enter the temperature value");

        io::stdin()
            .read_line(&mut temp_value)
            .expect("Failed to read line");

        let temp_value: f32 = match temp_value.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        break;
    }

    loop {
        println!("Fahrenheit or Celcius? [Ff/Cc]");

        io::stdin()
            .read_line(&mut temp_unit)
            .expect("Failed to read line");

        let temp_unit = temp_unit.trim().to_uppercase();

        if temp_unit == "F" || temp_unit == "C" {
            break;
        }
    }

    // convert values to what we want
    let temp_value: f32 = temp_value.trim().parse()
        .expect("Uhhh, how'd this become NaN?");
    let temp_unit = temp_unit.trim().to_uppercase();
    println!("{}{}", temp_value, temp_unit);

    if temp_unit == "F" {
        let converted_value = f_to_c(temp_value);
        println!("{}F = {}C", temp_value, converted_value);
    } else {
        let converted_value = c_to_f(temp_value);
        println!("{}C = {}F", temp_value, converted_value);
    }
}

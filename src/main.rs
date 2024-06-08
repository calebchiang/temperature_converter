use std::process;
use temperature_converter;
use temperature_converter::UserInput;

fn main() {
    match UserInput::get_user_input() {
        Ok(user_input) => {
            let result = user_input.convert_temp();
            println!("{}", result);
        }
        Err(e) => {
            println!("{}", e);
            process::exit(1);
        }
    }
}

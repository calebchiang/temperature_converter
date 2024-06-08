use std::io;

pub struct UserInput {
    choice: usize,
    temperature: f64,
}

impl UserInput {
    pub fn get_user_input() -> Result<UserInput, &'static str> {
        println!("Welcome to the temperature converter!");
        println!("Enter an option (1/2):");
        println!("1. Celsius to Fahrenheit");
        println!("2. Fahrenheit to Celsius");

        let mut choice_input = String::new();
        io::stdin().read_line(&mut choice_input).expect("Failed to read line");
        let choice: usize = choice_input.trim().parse().unwrap_or(0);
        if choice != 1 && choice != 2 {
            return Err("Invalid choice, please pick between 1 or 2");
        }

        println!("Enter the temperature to convert:");
        let mut temperature_input = String::new();
        io::stdin().read_line(&mut temperature_input).expect("Failed to read user input");
        let temperature: f64 = temperature_input.trim().parse()
            .map_err(|_| "Invalid temperature input; please enter a valid number.")?;

        Ok(UserInput { choice, temperature })
    }

    pub fn convert_temp(&self) -> String {
        match self.choice {
            1 => {
                let converted = self.temperature * 9.0 / 5.0 + 32.0;
                format!("{:.1}°C is {:.1}°F", self.temperature, converted)
            },
            2 => {
                let converted = (self.temperature - 32.0) * 5.0 / 9.0;
                format!("{:.1}°F is {:.1}°C", self.temperature, converted)
            },
            _ => "Invalid choice".to_string(),
        }
    }
}
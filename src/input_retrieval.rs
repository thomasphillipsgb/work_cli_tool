pub trait InputRetrieval {
    fn get_user_choice(&self, amount_of_inputs: i32) -> i32;
    fn get_freetype_input(&self, prompt: &str) ->  Result<String, Box<dyn std::error::Error>>;
}

pub struct ConsoleInputRetrieval;

impl InputRetrieval for ConsoleInputRetrieval {
    fn get_user_choice(&self, amount_of_inputs: i32) -> i32 {
        use std::io;

        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");
        match input.trim().parse::<i32>() {
            Ok(num) if (1..=amount_of_inputs).contains(&num) => num,
            _ => {
                println!("Invalid choice, please enter a number between 1 and {}.", amount_of_inputs);
                self.get_user_choice(amount_of_inputs)
            }
        }
    }

    fn get_freetype_input(&self, prompt: &str) -> Result<String, Box<dyn std::error::Error>> {
        use std::io;

        println!("{}", prompt);
        let mut input = String::new();
        io::stdin().read_line(&mut input)?;
        Ok(input.trim().to_string())
    }
}
use std::io;

mod menu {

    use std::io;
    use ansi_term::Color;

    pub fn get_input() -> io::Result<String> {
        let mut input = String::new();
        match io::stdin().read_line(&mut input) {
            Ok(_) => return Ok(input),
            Err(x) => return Err(x)
        }
    }

    pub fn ask_question(question: &str) -> io::Result<String> {
        println!("{}", question);
        get_input()
    }

    pub fn print_list(items: Vec<&str>) -> () {
        for n in 0..items.len() {
            println!("[{}] {}", Color::Yellow.paint(n.to_string()), items[n]);
        }
    }

    pub fn ask_list(items: Vec<&str>, question: &str) -> io::Result<String> {
        print_list(items);
        ask_question(question)
    }

    pub fn parse_unsigned(word: &str) -> Result<u32, std::num::ParseIntError> {
        word.parse::<u32>()
    }

    pub fn parse_float(word: &str) -> Result<f32, std::num::ParseFloatError> {
        word.parse::<f32>()
    }

}

mod recipe {

    #[derive(Debug)]
    pub struct Recipe {
        bulk_components: Vec<Ingredient>,
        micro_components: Option<Vec<Ingredient>>
    }

    impl Recipe {
        
        pub fn new(bulk_components: Vec<Ingredient>, micro_components: Option<Vec<Ingredient>>) -> Recipe {
            return Recipe { bulk_components: bulk_components, micro_components: micro_components };
        }

        pub fn get_bulk_components(&self) -> &Vec<Ingredient> {
            return &self.bulk_components;
        }

        pub fn get_micro_components(&self) -> &Option<Vec<Ingredient>> {
            return &self.micro_components;
        }

        // pub fn set_bulk_components(&self, bulk_components: Vec<Ingredient>) -> Self {
        //     let micro_components = self.get_micro_components();
        //     return Recipe { bulk_components: bulk_components, micro_components: micro_components }
        // }

    }

    #[derive(Debug)]
    pub struct Ingredient {
        name: String,
        percent: f32
    }

    impl Ingredient {

        pub fn new(name: String, percent: f32) -> Ingredient {
            return Ingredient { name: name, percent: percent }
        }

        pub fn get_name(&self) -> &str {
            return &self.name;
        }

        pub fn get_percent(&self) -> &f32 {
            return &self.percent;
        }

        pub fn set_name(&self, name: String) -> Self {
            return Self::new(name, *self.get_percent());
        }

        pub fn set_percent(&self, percent: f32) -> Self {
            return Self::new(self.get_name().to_string(), percent);
        }

    }

}

fn main() -> io::Result<()> {

    let question = "What is your ingredient name?";
    let input = menu::ask_question(&question).unwrap();
    let mut my_ingredient = recipe::Ingredient::new(input.trim().to_string(), 50f32);
    println!("You typed {}", my_ingredient.get_name());
    println!("Your ingredient is {} with {}%", my_ingredient.get_name(), my_ingredient.get_percent());

    my_ingredient = my_ingredient.set_percent(30f32);
    println!("But I think I'll have {} with {}% instead!", my_ingredient.get_name(), my_ingredient.get_percent());

    let test_list = vec!["crazy!", "i stay noided", "what the dog doin?"];
    let list_response = menu::ask_list(test_list, "Are you listening to fucking hyperpop you fucking furry? (type a number)");
    println!("You typed {}", list_response.as_ref().unwrap());
    match menu::parse_unsigned(list_response.as_ref().unwrap().as_str().trim()) {
        Ok(num) => println!("As an unsigned: {:?}", num),
        Err(_) => println!("Not an unsigned!")
    }
    match menu::parse_float(list_response.as_ref().unwrap().as_str().trim()) {
        Ok(num) => println!("As a float: {:?}", num),
        Err(_) => println!("Not a float!")
    }

    Ok(())

}

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

}

mod recipe {
    #[derive(Debug)]
    pub struct Ingredient {
        pub name: Box<String>,
        pub percent: Box<f32>
    }

    impl Ingredient {
        pub fn new(name: String, percent: f32) -> Ingredient {
            Ingredient { name: Box::new(name), percent: Box::new(percent) }
        }
    }
}

fn main() -> io::Result<()> {
    let question = "What is your ingredient name?";
    let input = menu::ask_question(&question).unwrap();
    let my_ingredient = recipe::Ingredient::new(input.trim().to_string(), 50f32);
    println!("You typed {}", my_ingredient.name);
    println!("Your ingredient is {} with {}%", my_ingredient.name, my_ingredient.percent);

    let test_list = vec!["crazy!", "i stay noided", "what the dog doin?"];
    let list_response = menu::ask_list(test_list, "Are you listening to fucking hyperpop you fucking furry?");
    println!("You typed {}", list_response?);

    Ok(())
}

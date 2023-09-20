use std::io;

#[derive(Debug)]
struct Ingredient {
    name: Box<String>,
    percent: Box<f32>
}

impl Ingredient {
    fn new(name: String, percent: f32) -> Ingredient {
        Ingredient { name: Box::new(name), percent: Box::new(percent) }
    }
}

fn get_input() -> io::Result<String> {
    let mut input = String::new();
    io::stdin().read_line(&mut input);
    return Ok(input);
}

fn main() -> io::Result<()> {
    let input = get_input().unwrap();
    let my_ingredient = Ingredient::new(String::from(input.trim()), 50f32);
    println!("You typed {}", my_ingredient.name);
    println!("Your ingredient is {} with {}%", my_ingredient.name, my_ingredient.percent);
    Ok(())
}

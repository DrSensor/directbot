mod directbot;

use reqwest::get;
use directbot::Response;
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let data: Response = get("https://api.noopschallenge.com/directbot?count=60&pattern=invader")?.json()?;
    println!("Noop Noop! \n {:?}", data);
    Ok(())
}

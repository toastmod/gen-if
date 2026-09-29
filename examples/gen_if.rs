use copilot_interceptor::prelude::tokio;
use gen_if::{Res, gen_if};

#[tokio::main]
async fn main() -> Res<()> {

    // User can enter any word...
    let mut my_color = String::new();
    std::io::stdin().read_line(&mut my_color)?;
    my_color = my_color.replace('\n',"").replace('\r', "");
    
    // Use gemma4 to evaluate if the statement is true or false.
    gen_if! { 
        ["gemma4"] if ("{my_color} is a color") {
            // The true branch...
            println!("{} is a color", my_color);
        } else {
            // The false branch...
            println!("{} is NOT a color", my_color);
        } catch {
            // If the request to llama.cpp encounters an error...
            println!("There was an error contacting llama.cpp");
        }
    };
    
    Ok(())
}
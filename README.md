# A Generative If Statement for Rust
*Why vibe code when you can code the vibes?*

This is a proof-of-concept just for fun inspired by [Jev](https://typesafe.ai/blog/introducing-system-one-models-and-jev).<br>Jev uses an evaluation model (which is better for this usecase) while this POC uses an LLM.

## What is this?
This is an if statement that requests a structured output (a single boolean) from an LLM, given some prompt string. Under the hood the input prompt isn't the user message but rather the description for the structured output's boolean parameter. 

## Example
```rust
use copilot_interceptor::prelude::tokio;
use gen_if::{GenIfConnection, Res, gen_if};

#[tokio::main]
async fn main() -> Res<()> {

    // Create a new connection to an OpenAI-compatible endpoint
    let gemma4 = GenIfConnection::new("http://localhost:11434", "gemma4");

    // User can enter any word...
    let mut my_color = String::new();
    std::io::stdin().read_line(&mut my_color)?;
    my_color = my_color.replace('\n',"").replace('\r', "");
    
    // Use gemma4 to evaluate if the statement is true or false.
    gen_if! { 
        [gemma4] if ("{my_color} is a color") {
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
```

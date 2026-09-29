use std::{error::Error, fmt};

use copilot_interceptor::{json_response_format, openai_client::{JsonSchema, ResponseFormat}, openai_server::OpenAiRequest, prelude::{HeaderMap, tokio}};
use serde_json::json;

mod generate;

pub type Res<T> = Result<T, Box<dyn Error>>;

#[derive(Debug)]
pub struct MyError(pub String);

// Implement Display so it can be formatted into a message string
impl fmt::Display for MyError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

// Implement the Error trait
impl Error for MyError {}

pub async fn gen_if(model: &str, clause: String) -> Res<bool> {
    if let Some(resp) = generate::generate(HeaderMap::new(), OpenAiRequest {
        model: model.to_string(),
        messages: vec![],
        temperature: Some(1.0),
        stream: false,
        response_format: Some(json_response_format!({
            name: "answer".to_string(),
            strict: true,
            schema: json!({
                "type": "boolean",
                "description": &clause
            })

        }))
    }, None).await? {
        let out = resp.choices[0].delta.as_ref().unwrap().content.clone().unwrap();
        let r = serde_json::from_str::<bool>(&out)?;
        Ok(r)
    } else {
        Err(Box::new(MyError(format!("Could not connect to service!"))))
    }

}

#[macro_export]
macro_rules! gen_if {
    ([$model:expr] if ($($arg:tt)*) $body:block else $elseb:block catch $ebody:block) => {
        if let Ok(_result) = gen_if($model, format!($($arg)*)).await {
            if _result {$body} else {$elseb}
        } else $ebody
    }
}

mod test {
    use copilot_interceptor::prelude::tokio;
    use crate::{Res, gen_if};

    #[tokio::test]
    async fn gen_if_works() -> Res<()> {
        gen_if!(["gemma4"] if ("blue is a color") {
            println!("blue is in fact a color");
        } else {
            println!("blue is not a color");
        } catch {

        });
    
        Ok(())
    }

}

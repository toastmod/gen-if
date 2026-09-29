use std::{ convert::Infallible, io::Read, vec };

use copilot_interceptor::{
    openai_server::{ OpenAiRequest, OpenAiResponse },
    prelude::{
        Client,
        Event,
        Eventsource,
        HeaderMap,
        StreamExt,
        reqwest::{ Error, Response },
        tokio::sync::mpsc::Sender,
    },
};
use serde_json::json;

pub async fn generate(
    host: &str,
    headers: HeaderMap,
    mut request_body: OpenAiRequest,
    stream_tx: Option<&Sender<Result<Event, Infallible>>>
) -> Result<Option<OpenAiResponse>, Error> {
    let client = Client::new();
    if let Some(tx) = stream_tx {
        request_body.stream = true;
        match
            client
                .post(format!("{}/v1/chat/completions", host))
                .headers(headers)
                .header("Connection", "keep-alive")
                .json(&request_body)
                .send().await
        {
            Ok(response) => {
                let mut data = response.bytes_stream().eventsource();
                while let Some(Ok(x)) = data.next().await {
                    let event = Event::default().event(x.event).data(x.data).id(x.id); // TODO: impl retry and friends
                    let _ = tx.send(Ok(event)).await;
                }
            }
            Err(e) => {
                println!("Error sending request to llama.cpp: {:?}", e);
                let event = Event::default().data(format!("Error connecting to backend: {}", e));
                let _ = tx.send(Ok(event)).await;
            }
        }
        Ok(None)
    } else {
        request_body.stream = false;
        match
            client
                .post("http://desktop-ttjki31:10000/v1/chat/completions")
                .headers(headers)
                .header("Connection", "keep-alive")
                .json(&request_body)
                .send().await
        {
            Ok(response) => {
                let data: OpenAiResponse = response.json().await.unwrap();
                Ok(Some(data))
            }
            Err(e) => {
                println!("Error sending request to llama.cpp: {:?}", e);
                let event = Event::default().data(format!("Error connecting to backend: {}", e));
                Ok(None)
            }
        }
    }
}

pub async fn embed(input: String) -> Result<Vec<f32>, Error> {
    let client = Client::new();
    match
        client
            .post("http://desktop-ttjki31:10000/v1/embeddings")
            .json(&json!({ "input": input }))
            .send().await
    {
        Ok(response) => {
            let data: serde_json::Value = response.json().await.unwrap();
            Ok(
                data
                    .as_object()
                    .unwrap()
                    .get("data")
                    .unwrap()
                    .as_array()
                    .unwrap()[0]
                    .as_object()
                    .unwrap()
                    .get("embedding")
                    .unwrap()
                    .as_array()
                    .unwrap()
                    .iter()
                    .map(|v| v.as_f64().unwrap() as f32)
                    .collect()
            )
        }
        Err(e) => {
            println!("Error sending request to llama.cpp: {:?}", e);
            let event = Event::default().data(format!("Error connecting to backend: {}", e));
            Ok(vec![])
        }
    }
}

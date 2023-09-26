use crate::chi_models::{Action, map_to_headermap};
use reqwest::Client;

pub async fn send_request(request: Action) -> Result<reqwest::Response, reqwest::Error> {
  if request.method == "POST" {
      println!("{:?}", request);
      send_post(request).await
  } else if request.method == "PUT" {
      send_put(request).await
  } else if request.method == "PUT" {
      send_delete(request).await
  } else {
      send_get(request).await
  }
}

pub async fn send_get(request: Action) -> Result<reqwest::Response, reqwest::Error> {
  let body = reqwest::get(request.url).await;
  body
}


pub async fn send_post(request: Action) -> Result<reqwest::Response, reqwest::Error> {
  let client = Client::new();
  
  let response = client
      .post(request.url)
      .headers(map_to_headermap(request.headers))
      .body(request.body)
      .send()
      .await;
  response
}


pub async fn send_put(request: Action) -> Result<reqwest::Response, reqwest::Error>  {
  let client = Client::new();
  
  let response = client
      .put(request.url)
      .headers(map_to_headermap(request.headers))
      .body(request.body)
      .send()
      .await;
  response
}

pub async fn send_delete(request: Action) -> Result<reqwest::Response, reqwest::Error> {
  let client = Client::new();
  
  let response = client
      .delete(request.url)
      .headers(map_to_headermap(request.headers))
      .body(request.body)
      .send()
      .await;
  response
}

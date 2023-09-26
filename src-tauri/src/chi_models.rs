use reqwest::{Error, Response};
use reqwest::header::{HeaderMap, HeaderValue, HeaderName};
use serde::{Serialize, Deserialize};
use std::str::FromStr;


// export type LibraryNode = {
//   id: string;
//   name: string;
//   actions?: Action[];
//   children?: LibraryNode[];
// };

#[derive(Serialize, Deserialize, Debug)]
pub struct ChiLibraryNode {
  pub id: String,
  pub name: String,
  pub actions: Vec<Action>,
  pub children: Vec<ChiLibraryNode>
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ChiRequest {
    pub method: String,
    pub url: String,
}

#[derive(Serialize)]
pub struct ChiConfig {
    pub methods: Vec<String>,
    pub libraries: Vec<ChiLibraryNode>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct EnabledKvp<TKey, TVal> {
    is_enabled: bool,
    key: TKey,
    value: TVal,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Action {
    pub id: String,
    pub method: String,
    pub url: String,
    pub name: String,
    pub headers: Vec<EnabledKvp<String, String>>,
    pub query: Vec<EnabledKvp<String, String>>,
    pub path: Vec<EnabledKvp<String, String>>,
    pub body: String
}

impl<TKey, TVal> EnabledKvp<TKey, TVal> {
  pub fn try_kvp(&self) -> Option<(&TKey, &TVal)> {
    if self.is_enabled {
      Some((&self.key, &self.value))
    } else {
      None
    }
  }
}

pub fn map_to_headermap(headers: Vec<EnabledKvp<String, String>>) -> HeaderMap {
  let mut header_map = HeaderMap::new();
  let kvps = headers
    .iter()
    .map(|kvp| kvp.try_kvp())
    .filter_map(|x| x)
    .map(|(key, val)| (HeaderName::from_str(&key), HeaderValue::from_str(&val)));

  for (key, value) in kvps {
    if let Ok(name) = key {
      if let Ok(val) = value {
        header_map.insert(name, val);
      }
    }
  }
  
  header_map
}


#[derive(Serialize, Deserialize, Debug)]
pub struct SerializableError {
    // Add fields from the Error struct that you want to serialize
    // For example, you can include fields like source, url, etc.
    // For simplicity, let's assume we are only serializing the error message.
    message: String,
}

impl From<Error> for SerializableError {
    fn from(error: Error) -> Self {
        // Extract the necessary information from the error and populate the fields
        let message = error.to_string();
        
        SerializableError { message }
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct SerializableResponse {
    // Add fields from the Response struct that you want to serialize
    // For example, you can include fields like status, headers, etc.
    // For simplicity, let's assume we are only serializing the status code.
    status: u16,
    headers: Vec<(String, String)>,
    body: String
}

impl From<Response> for SerializableResponse {
    fn from(response: Response) -> Self {
        // Extract the necessary information from the response and populate the fields
        let status = response.status().as_u16();
        let headers = response
            .headers()
            .iter()
            .map(|(name, value)| (String::from(name.to_string()), String::from(value.to_str().unwrap_or(""))))
            .collect();

        SerializableResponse { 
            status,
            headers,
            body: "".to_string()
        }
    }
}

pub async fn map_to_serializable_response(response: Response) -> SerializableResponse {
        // Extract the necessary information from the response and populate the fields
        let status = response.status().as_u16();
        let headers = response
            .headers()
            .iter()
            .map(|(name, value)| (String::from(name.to_string()), String::from(value.to_str().unwrap_or(""))))
            .collect();
        let body = response.text().await.expect("");

        SerializableResponse { 
            status,
            headers,
            body
        }
}

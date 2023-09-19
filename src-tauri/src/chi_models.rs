use std::str::FromStr;

use reqwest::header::{HeaderMap, HeaderValue, HeaderName};
use serde::{Serialize, Deserialize};



#[derive(Serialize, Deserialize, Debug)]
pub struct ChiRequest {
    pub method: String,
    pub url: String,
}

#[derive(Serialize)]
pub struct ChiConfig {
    pub methods: Vec<String>
}

#[derive(Serialize, Deserialize, Debug)]
pub struct EnabledKvp<TKey, TVal> {
    is_enabled: bool,
    key: TKey,
    value: TVal,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Action {
    pub method: String,
    pub url: String,
    pub name: String,
    pub headers: Option<Vec<EnabledKvp<String, String>>>,
    pub query: Option<Vec<EnabledKvp<String, String>>>,
    pub path: Option<Vec<EnabledKvp<String, String>>>,
    pub body: String
}

impl<TKey, TVal> EnabledKvp<TKey, TVal> {
  pub fn try_kvp(&self) -> Option<(&TKey, &TVal)> {
    if(self.is_enabled) {
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

pub fn map_to_header_map(headers: Option<Vec<EnabledKvp<String, String>>>) -> HeaderMap {
    let mut header_map = HeaderMap::new();
    
    if let Some(enabled_kvps) = headers {
        for kvp in enabled_kvps {
            if kvp.is_enabled {
                if let Ok(header_value) = HeaderValue::from_str(&kvp.value) {
                    if let Ok(header_name) = HeaderName::from_str(&kvp.key) {
                        header_map.insert(header_name, header_value);
                    }
                }
            }
        }
    }
    
    header_map
}
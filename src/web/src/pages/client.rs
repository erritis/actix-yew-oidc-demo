use std::collections::HashMap;

use serde_json::Value;
use yew::prelude::*;
use yew_oauth2::prelude::{use_auth_state, OAuth2Context};


fn set_dict(data: Value) -> HashMap<String, String> {
  // Создаем HashMap<String, String> и заполняем его значениями из JSON
  let mut hashmap = HashMap::new();
  if let Some(obj) = data.as_object() {
      for (key, value) in obj.iter() {
          if let Some(string_value) = value.as_str() {
              hashmap.insert(key.to_owned(), string_value.to_owned());
          }
      }
  }
  hashmap
}


fn get_dict(auth: OAuth2Context) -> serde_json::error::Result<HashMap<String, String>, > {
  let claims = auth.claims().unwrap();

  let json = serde_json::to_string(claims)?;

  // Парсим JSON в структуру Value
  let data: Value = serde_json::from_str(&json)?;
  
  Ok(set_dict(data))
}


#[function_component(ClientPage)]
pub fn client() -> Html {
  let auth: OAuth2Context = use_auth_state().expect("Requires OAuth2Context component in parent hierarchy");

  let dict = get_dict(auth).expect("serde for claims is error");

  let claims = dict.iter().map(|(key, value)| html! {
    <tr>
      <td>{ key }</td>
      <td>{ value }</td>
    </tr>
  }).collect::<Html>();


  html! {
    <div>
      <h1 id="tableLabel">{"Client Side Page"}</h1>

      <p>{"This page displays what can be obtained using oidc client-side authorization."}</p>

      <p>{"Userinfo:"}</p>

      <table class="table table-striped" aria-labelledby="tableLabel">
          <thead>
            <tr>
              <th>{"Key:"}</th>
              <th>{"Value"}</th>
            </tr>
          </thead>
          <tbody>
            { claims }
          </tbody>
        </table>
    </div>
  }
}

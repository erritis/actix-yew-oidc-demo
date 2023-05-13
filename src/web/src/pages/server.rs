use std::collections::HashMap;

use gloo_net::http::Request;
use serde_json::Value;
use yew::prelude::*;
use yew_oauth2::prelude::{OAuth2Context, use_auth_state};

use crate::{opts::Opts, services::config::use_config};


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


#[function_component(ServerPage)]
pub fn server() -> Html {

  let auth: OAuth2Context = use_auth_state().expect("Requires OAuth2Context component in parent hierarchy");
  let opts: Opts = use_config();

  let token = auth.access_token().unwrap_or("").to_owned();

  let dict = use_state(|| HashMap::<String, String>::new());
  {
      let dict = dict.clone();
      use_effect_with_deps(move |_| {
          let dict = dict.clone();
          wasm_bindgen_futures::spawn_local(async move {
              let value: Value = Request::get(&format!("{}/userinfo", &opts.backend_url))
                  .header("Authorization", &format!("Bearer {}", token))
                  .send()
                  .await
                  .unwrap()
                  .json()
                  .await
                  .unwrap();
              dict.set(set_dict(value))
          });
          || ()
      }, ());
  };

  let claims = dict.iter().map(|(key, value)| html! {
    <tr>
      <td>{ key }</td>
      <td>{ value }</td>
    </tr>
  }).collect::<Html>();

  html! {
    <div>
      <h1 id="tableLabel">{"Server Side Page"}</h1>

      <p>{"This page displays what can be obtained using oidc authorization from the server side."}</p>

      <p>{"This component demonstrates fetching data from the server."}</p>

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

use tauri_sys::tauri::{invoke, ResponseError};
use yew::{platform::spawn_local, prelude::*};

#[function_component]
fn HelloWorld() -> Html {
  html! { "Hello world" }
}

#[function_component]
fn Counter() -> Html {
  let count = use_state(|| 0);
  let onclick = {
    let count = count.clone();
    move |_| {
      let value = *count + 1;
      count.set(value);
    }
  };

  let onclick_tauri_event = {
    move |_| {
      spawn_local(async {
        let res = invoke::<_, String, String>("hello", &()).await;
        match res {
          Ok(data) => {
            log::info!("Success tauri command: {}", data);
          }
          Err(e) => {
            if let ResponseError::ReceivedError { error } = e {
              log::error!("Error tauri command: {}", error);
            }
          }
        }
      })
    }
  };

  html! {
      <div>
        <div>
            {*count}
        </div>
        <div>
        <button onclick={onclick}>{ "Increment" }</button>
        </div>
        <div>
        <button onclick={onclick_tauri_event}>{ "click Tauri Event" }</button>
        </div>
      </div>
  }
}

// Then somewhere else you can use the component inside `html!`
#[function_component]
fn App() -> Html {
  html! {
      <>
        <HelloWorld />
        <Counter />
      </>
  }
}

fn main() {
  wasm_logger::init(wasm_logger::Config::default());
  yew::Renderer::<App>::new().render();
}

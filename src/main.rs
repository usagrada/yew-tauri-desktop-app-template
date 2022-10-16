use yew::prelude::*;

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

  html! {
      <div>
        <div>
            {*count}
        </div>
        <button onclick={onclick}>{ "Increment" }</button>
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
  yew::Renderer::<App>::new().render();
}

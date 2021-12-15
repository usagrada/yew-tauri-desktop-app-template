use crate::layout::Layout;
use yew::prelude::*;
use yew_router::prelude::*;

#[derive(Debug, Clone, Copy, PartialEq, Routable)]
pub enum MyRoute {
	#[at("/")]
	Home,
	#[at("/about")]
	About,
	#[not_found]
	#[at("/404")]
	NotFound,
}

pub struct Model {}

impl Component for Model {
	type Message = ();
	type Properties = ();

	fn create(_ctx: &Context<Self>) -> Self {
		Self {}
	}

	fn update(&mut self, _ctx: &Context<Self>, _msg: Self::Message) -> bool {
		false
	}

	fn view(&self, _ctx: &Context<Self>) -> Html {
		html! {
			<BrowserRouter>
				<Layout>
					<Switch<MyRoute> render={Switch::render(switch)} />
				</Layout>
			</BrowserRouter>
		}
	}
}

fn switch(routes: &MyRoute) -> Html {
	match routes {
		MyRoute::Home => {
			html! {<div>{"Home"}</div>}
		}
		MyRoute::About => {
			html! {<div>{"About"}</div>}
		}
		_ => {
			html! {<div>{"404 Not Found"}</div>}
		}
	}
}

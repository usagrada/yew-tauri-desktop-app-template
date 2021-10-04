use yew::prelude::*;
pub mod footer;
pub mod header;
use footer::Footer;
use header::Header;


pub struct Layout {
	// props: LayoutProps,
}

#[derive(PartialEq, Properties)]
pub struct LayoutProps {
	pub children: Children,
}

impl Component for Layout {
	type Message = ();
	type Properties = LayoutProps;

	fn create( _ctx: &Context<Self>) -> Self {
		Self { }
	}

	fn update(&mut self, _ctx: &Context<Self>, _msg: Self::Message) -> bool {
		false
	}

	fn view(&self, _ctx: &Context<Self>) -> Html {
		html! {
			<>
				<Header />
				<main>
				{ _ctx.props().children.clone() }
				</main>
				<Footer />
			</>
		}
	}
}

use yew::prelude::*;
pub mod footer;
pub mod header;
use footer::Footer;
use header::Header;


pub struct Layout {
	props: LayoutProps,
	_link: ComponentLink<Self>,
}

#[derive(Properties, Clone)]
pub struct LayoutProps {
	pub children: Children,
}

impl Component for Layout {
	type Message = ();
	type Properties = LayoutProps;

	fn create(props: Self::Properties, _link: ComponentLink<Self>) -> Self {
		Self { props, _link }
	}

	fn update(&mut self, _msg: Self::Message) -> ShouldRender {
		false
	}

	fn change(&mut self, _props: Self::Properties) -> ShouldRender {
		// Should only return "true" if new properties are different to
		// previously received properties.
		// This component has no properties so we will always return "false".
		false
	}

	fn view(&self) -> Html {
		html! {
			<>
				<Header />
				<main>
				{ self.props.children.clone() }
				</main>
				<Footer />
			</>
		}
	}
}

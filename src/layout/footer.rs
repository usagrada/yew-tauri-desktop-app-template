use yew::prelude::*;

pub struct Footer {
    // props: Props,
}

impl Component for Footer {
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
            <div>
                {"footer"}
            </div>
        }
    }
}

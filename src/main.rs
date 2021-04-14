#![recursion_limit = "10000"]

mod thought;
mod timeline;

use yew::prelude::*;
use yew_router::prelude::*;

#[derive(Switch, Debug, Clone)]
enum AppRoute {
    #[to = "/test"]
    Test,
    #[to = "/"]
    Index,
}

enum Msg {}

struct Model {
    // `ComponentLink` is like a reference to a component.
    // It can be used to send messages to the component
    link: ComponentLink<Self>,
}
impl Component for Model {
    type Message = Msg;
    type Properties = ();

    fn create(_props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self { link }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {}
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        // Should only return "true" if new properties are different to
        // previously received properties.
        // This component has no properties so we will always return "false".
        false
    }

    fn rendered(&mut self, first_render: bool) {
        if first_render {}
    }

    fn view(&self) -> Html {
        html! {
            <body>
                <div class="container mx-auto">
                  <div class="border-b-2 mb-2">
                    <Router<AppRoute, ()>
                        render = Router::render(|switch: AppRoute| {
                            match switch {
                                AppRoute::Test => html!{
                                <RouterAnchor<AppRoute>
                                    route=AppRoute::Index
                                    >
                                    { "Index" }
                                </RouterAnchor<AppRoute>>
                                },
                                AppRoute::Index => html! {
                                <div>
                                    <RouterAnchor<AppRoute>
                                        route=AppRoute::Test
                                        >
                                        { "Test" }
                                    </RouterAnchor<AppRoute>>
                                </div>
                                }
                            }
                        })
                    />
                  </div>

                  <timeline::Timeline />
                </div>
            </body>
        }
    }
}

fn main() {
    yew::start_app::<Model>();
}

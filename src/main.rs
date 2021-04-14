#![recursion_limit = "10000"]
use postgrest::Postgrest;
use yew::prelude::*;
use yew_router::prelude::*;
use yewtil::future::LinkFuture;

#[derive(Switch, Debug, Clone)]
enum AppRoute {
    #[to = "/test"]
    Test,
    #[to = "/"]
    Index,
}

enum Msg {
    LoadInfo,
    SetInfo(Result<String, reqwest::Error>),
}

struct Model {
    // `ComponentLink` is like a reference to a component.
    // It can be used to send messages to the component
    link: ComponentLink<Self>,
    value: i64,
    msg: String,
}

// Sample code to talk to postgrest, exposed at :7000
// TODO: Refactor and put in module.
async fn fetch_data() -> Msg {
    let client = Postgrest::new("http://localhost:7000");
    let mresp = client.from("thought").select("*").execute().await;
    let v = match mresp {
        Err(err) => Err(err),
        Ok(resp) => match resp.text().await {
            Err(err) => Err(err),
            Ok(v) => Ok(v),
        },
    };
    Msg::SetInfo(v)
}

impl Component for Model {
    type Message = Msg;
    type Properties = ();

    fn create(_props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self {
            link,
            value: 0,
            msg: "...".to_string(),
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::LoadInfo => {
                let data_future = fetch_data();
                self.link.send_future(data_future);
                true
            }
            Msg::SetInfo(response) => {
                match response {
                    Ok(s) => self.msg = s,
                    Err(error) => self.msg = error.to_string(),
                };
                true
            }
        }
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        // Should only return "true" if new properties are different to
        // previously received properties.
        // This component has no properties so we will always return "false".
        false
    }

    fn rendered(&mut self, first_render: bool) {
        if first_render {
            self.link.send_message(Msg::LoadInfo)
        }
    }

    fn view(&self) -> Html {
        html! {
            <body>
                <div class="container mx-auto">
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

                    <div class="border-t-1">
                        <button
                            class="border-2 rounded p-2 bg-purple-200"
                            onclick=self.link.callback(|_|
                            Msg::LoadInfo)>{ "Refresh" }</button>
                        <p>{ self.value }</p>
                        <div class="font-mono overflow">
                            { self.msg.clone() }
                        </div>
                    </div>

                </div>
            </body>
        }
    }
}

fn main() {
    yew::start_app::<Model>();
}

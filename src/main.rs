use yew::services::fetch::{Request, Response};
use yew::{format::Nothing, services::FetchService};
use yew::{prelude::*, services::fetch::FetchTask};

enum Msg {
    AddOne,
    SetInfo(Result<String, anyhow::Error>),
}

struct Model {
    // `ComponentLink` is like a reference to a component.
    // It can be used to send messages to the component
    link: ComponentLink<Self>,
    value: i64,
    msg: String,
    fetch_task: Option<FetchTask>,
}

impl Component for Model {
    type Message = Msg;
    type Properties = ();

    fn create(_props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self {
            link,
            value: 0,
            msg: "...".to_string(),
            fetch_task: None,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::AddOne => {
                // Playing with network fetch ...
                let url = "https://www.reddit.com/r/TheMotte/top.json";
                let request = Request::get(url)
                    .body(Nothing)
                    .expect("Could not build that request");
                let callback = self
                    .link
                    .callback(|rsp: Response<_>| Msg::SetInfo(rsp.into_body()));
                let task = FetchService::fetch(request, callback).expect("failed to start request");
                self.fetch_task = Some(task);
                self.value += 1;
                // the value has changed so we need to
                // re-render for it to appear on the page
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

    fn view(&self) -> Html {
        html! {
            <body>
                <div class="container mx-auto">
                    <div>
                        <button
                            class="border-2 rounded p-2 bg-purple-200"
                            onclick=self.link.callback(|_| Msg::AddOne)>{ "Fetch something" }</button>
                        <p>{ self.value }</p>
                        <div class="monospace overflow">
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

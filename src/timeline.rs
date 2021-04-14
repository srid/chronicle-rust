use crate::thought::Thought;
use postgrest::Postgrest;
use yew::prelude::*;
use yewtil::future::LinkFuture;

pub enum Msg {
    LoadInfo,
    SetInfo(Result<Vec<Thought>, reqwest::Error>),
}

pub struct Timeline {
    link: ComponentLink<Self>,
    thoughts: Vec<Thought>,
    // FIXME Use Result type with data (thoughts)
    error: Option<reqwest::Error>,
}

impl Component for Timeline {
    type Message = Msg;
    type Properties = ();

    fn create(_props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self {
            link,
            thoughts: vec![],
            error: None,
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
                    Ok(s) => {
                        self.thoughts = s;
                        self.error = None;
                    }
                    Err(error) => {
                        self.thoughts = vec![];
                        self.error = Some(error);
                    }
                };
                true
            }
        }
    }

    fn change(&mut self, (): Self::Properties) -> ShouldRender {
        false
    }

    fn rendered(&mut self, first_render: bool) {
        if first_render {
            self.link.send_message(Msg::LoadInfo)
        }
    }

    fn view(&self) -> Html {
        html! {
            <div class="border-t-1">
                <button
                    class="border-2 rounded p-2 bg-purple-200"
                    onclick=self.link.callback(|_|
                    Msg::LoadInfo)>{ "Refresh" }</button>
                <div class="overflow">
                    { for self.thoughts.iter().map(|thought| {
                        html! {
                            <Thought thought=thought />
                        }
                      })
                    }
                </div>
            </div>
        }
    }
}

// Sample code to talk to postgrest, exposed at :7000
// TODO: Refactor and put in module.
async fn fetch_data() -> Msg {
    let client = Postgrest::new("http://localhost:7000");
    let mresp = client.from("thought").select("*").execute().await;
    let v = match mresp {
        Err(err) => Err(err),
        Ok(resp) => match resp.json::<Vec<Thought>>().await {
            Err(err) => Err(err),
            Ok(v) => Ok(v),
        },
    };
    Msg::SetInfo(v)
}

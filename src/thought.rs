use chrono::prelude::*;
use serde::Deserialize;
use uuid::Uuid;
use validator::Validate;
use yew::prelude::*;

#[derive(PartialEq, Deserialize, Clone, Debug, Validate, Properties)]
pub struct Thought {
    id: Uuid,
    #[validate(length(min = 4))]
    content: String,
    created: DateTime<Utc>,
}

#[derive(Clone, Debug, Properties)]
pub struct Props {
    pub thought: Thought,
}

impl Component for Thought {
    type Message = ();
    type Properties = Props;

    fn create(props: Self::Properties, _link: ComponentLink<Self>) -> Self {
        props.thought
    }

    fn update(&mut self, (): Self::Message) -> ShouldRender {
        false
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        self != &props.thought
    }

    fn view(&self) -> Html {
        html! {
            <div class="border-1 rounded p-2 mb-2">
                <div class="text-gray-200"> { self.id.to_simple() } </div>
                <div class="text-gray-400"> { self.created.to_string() } </div>
                <div> { self.content.clone() }</div>
            </div>
        }
    }

    fn rendered(&mut self, _first_render: bool) {}

    fn destroy(&mut self) {}
}

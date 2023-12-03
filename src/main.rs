use web_sys::HtmlInputElement;
use yew::html::Scope;
use yew::prelude::*;
use yew::{Component, Context};
mod model;

use model::Model;
pub enum Msg {
    Add(String),
    Remove(usize),
    RemoveAll,
    Nothing,
}

pub struct App {
    model: Model,
}

impl Component for App {
    type Message = Msg;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self {
            model: Model::new(),
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::Add(value) => self.model.add(&value),
            _ => {}
        };
        true
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        html! {
            <div>
                <h1> {"ToDo App"} </h1>
                {self.view_input(ctx.link())}
                // <button {onclick}>{ "+1" }</button>
                // <p>{ *counter }</p>
                <footer class="info">
                    <p>{ "Double-click to edit a todo" }</p>
                    <p>{ "Written by " }<a href="https://github.com/spayot/" target="_blank">{ "Sylvain Payot" }</a></p>
                </footer>
            </div>
        }
    }
}

impl App {
    fn view_input(&self, link: &Scope<Self>) -> Html {
        let onkeypress = link.batch_callback(|e: KeyboardEvent| {
            if e.key() == "Enter" {
                let input: HtmlInputElement = e.target_unchecked_into();
                let value = input.value();
                input.set_value("");
                Some(Msg::Add(value))
            } else {
                None
            }
        });
        html!(
            <input
                placeholder="what do you want to do?"
                {onkeypress}
            />
        )
    }
}

#[function_component]
fn App2() -> Html {
    let state = Model::new();
    let counter = use_state(|| 0);
    let onclick = {
        let counter = counter.clone();
        move |_| {
            let value = *counter + 1;
            counter.set(value);
        }
    };

    html! {
        <div>
            <h1> {"ToDo App"} </h1>
            // <input
            //     placeholder="what do you want to do?",
            //     value=
            // />
            <button {onclick}>{ "+1" }</button>
            <p>{ *counter }</p>
            <footer class="info">
                <p>{ "Double-click to edit a todo" }</p>
                <p>{ "Written by " }<a href="https://github.com/spayot/" target="_blank">{ "Sylvain Payot" }</a></p>
            </footer>
        </div>
    }

    // html! {
    //     <div>
    //         <h1> {"ToDo App"} </h1>
    //         <input
    //             placeholder="what do you want to do?",
    //             value=
    //         />
    //     </div>
    // }
}

fn main() {
    yew::Renderer::<App>::new().render();
}

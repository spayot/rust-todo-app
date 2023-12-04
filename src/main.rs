use web_sys::HtmlInputElement;
use yew::html::Scope;
use yew::prelude::*;
use yew::{Component, Context};
mod model;

use model::{Model, ToDo};
pub enum Msg {
    Add(String),
    Remove(usize),
    RemoveAll,
    Toggle(usize),
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
            Msg::Remove(idx) => self.model.remove(idx),
            Msg::RemoveAll => self.model.remove_all(),
            Msg::Toggle(idx) => self.model.toggle(idx),
        };
        true
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        html! {
            <div>
                <section class="todoapp">
                    <header class="header">
                        <h1> {"ToDo App"} </h1>
                        {self.view_input(ctx.link())}
                    </header>
                    <ul class="todo-list">
                        {for self.model.todos.iter().enumerate().map(|e| self.view_todo(e, ctx.link()))}
                    </ul>
                    <div>
                        <li>
                            <text>{format!("\tTasks Count: {}", self.model.total())}</text>
                            <button
                                class="clear-completed"
                                onclick= {ctx.link().callback(|_| {Msg::RemoveAll})} width=30>
                                <b> {"Delete All ToDos"}</b>
                            </button>
                        </li>
                    </div>
                </section>
                {self.view_footer()}
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
            <li>
            <input class="new-todo"
                placeholder="what do you want to do?"
                {onkeypress}
            />
            </li>
        )
    }

    fn view_todo(&self, (idx, todo): (usize, &ToDo), link: &Scope<Self>) -> Html {
        html! {
            <li>
                <div class="view">
                    <input
                        type="checkbox"
                        class="toggle"
                        checked={todo.completed}
                        onclick={link.callback(move |_| Msg::Toggle(idx))}
                    />
                    <label> {
                        if todo.completed {
                            html! {<s>{&todo.text}</s>}
                        } else {
                            html! {&todo.text}
                        }
                    } </label>
                    <button class="destroy" onclick={link.callback(move |_| Msg::Remove(idx))} />
                </div>
            </li>
        }
    }

    fn view_footer(&self) -> Html {
        html! {
            <footer class="info">
                <p>{ "Click on toggle to mark task as complete." }</p>
                <p>{ "Click on X to delete task." }</p>
                <p>{ "Written by " }<a href="https://github.com/spayot/" target="_blank">{ "Sylvain Payot" }</a></p>
            </footer>
        }
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}

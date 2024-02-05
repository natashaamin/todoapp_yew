use gloo::storage::{LocalStorage, Storage};
use state::{Entry, Filter, State};
use strum::IntoEnumIterator;
use web_sys::HtmlInputElement as InputElement;
use yew::events::{FocusEvent, KeyboardEvent};
use yew::html::Scope;
use yew::{classes, html, Classes, Component, Context, Html, NodeRef, TargetCast};

mod state;

const KEY: &str = "yew.todo.self";

pub enum Msg {
    Add(String)
}

pub struct App {
    state: State,
    focus_ref: NodeRef
}

impl Component for App {
    type Message = Msg;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        let entries = LocalStorage::get(KEY).unwrap_or_else(|_| Vec::new());
        let state = State {
            entries,
            filter: Filter::All
        };
        let focus_ref = NodeRef::default();
        Self { state, focus_ref }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let hidden_class = if self.state.entries.is_empty() {
            "hidden"
        } else {
            ""
        };
        html! {
            <div class="todomvc-wrapper">
                <section class="todoapp">
                    <header class="header">
                        <h1>{"todo"}</h1>
                        { self.view_input(ctx.link() )}
                    </header>
                </section>
            </div>
        }
    }
}

impl App {
    fn view_input(&self, link: &Scope<Self>) -> Html {
        let onkeypress = link.batch_callback(|e: KeyboardEvent| {
            if e.key() == "Enter" {
                let input: InputElement = e.target_unchecked_into();
                let value = input.value();
                input.set_value("");
                Some(Msg::Add(value))
            } else {
                None
            }
        });

        html! {
            <>
            <input
                class="new-todo"
                placeholder="What needs to be done?"
                {onkeypress}
            />
            <ul>
                
            </ul>
            </>
        }
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
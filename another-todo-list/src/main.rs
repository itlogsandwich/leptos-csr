use leptos::prelude::*;
use leptos::mount::mount_to_body;

pub struct Todo 
{
    id: i64,
    title: String,
    description: String,
    isComplete: bool,
}

#[component]
pub fn ListDisplay(title: ReadSignal<String>) -> impl IntoView
{
    view!
    {
        <div class="list">
            {{title}}
        </div>
    }
}

#[component]
pub fn App() -> impl IntoView
{

    // The differing Signal instantiaion is intentional.
    // It's done for practice

    let title_signal = RwSignal::<String>::new(String::from(""));
    let (title, set_title) = title_signal.split();

    let description_signal = RwSignal::new(String::from(""));
    let (description, set_description) = description_signal.split();

    let (status, set_status) = signal(false);

    view!
    {
        <main>
            <div class="container">
                <div class="input-divs">
                    <label for="title-input"> "What shall we do today?" </label>
                    <input id="title-input" type="text" bind:value=(title, set_title)/>
                </div>
                <div class="input-divs">
                    <label for="description-input"> "Add more details to it fr" </label>
                    <input id="description-input" type="text" bind:value=(description, set_description)/>
                </div>

                <ListDisplay title=title/>
            </div>
        </main>
    }
}

fn main() 
{
    mount_to_body(App);
}

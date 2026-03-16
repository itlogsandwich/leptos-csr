use leptos::prelude::*;
use leptos::mount::mount_to_body;

#[derive(Debug, Clone, Default)]
pub struct Todo 
{
    id: i64,
    title: String,
    description: String,
    is_complete: bool,
}

#[component]
pub fn ListDisplay(list: ReadSignal<Vec<Todo>>) -> impl IntoView
{
    view!
    {
        <For
            each=move || list.get()
            key= |list| list.id
            children = move |list|
            {
                let status = if list.is_complete { "Finished" } else {"Unfinished"};

                view!
                {

                <div>
                    <p>{list.title}</p>
                    <p>{list.description}</p> 
                    <p>{status}</p>
                </div>
                }
            }
        />
    }
}

#[component]
pub fn App() -> impl IntoView
{

    // The differing Signal instantiaion is intentional.
    // It's done for practice

    let (count, set_count) = signal(0);

    let title_signal = RwSignal::<String>::new(String::from(""));
    let (title, set_title) = title_signal.split();

    let description_signal = RwSignal::new(String::from(""));
    let (description, set_description) = description_signal.split();

    let (status, set_status) = signal(false);

    //this basically creates a Read Write signal and instantly splits it
    //or I can just do signal(Vec::<Todo>::new()); and it'll work lol
    let (list, set_list) = RwSignal::new(Vec::<Todo>::new()).split();


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
                <div>
                    <button type="button"
                    on:click=move |_| 
                    {
                        set_count.set(count.get() + 1);
                        set_list.write().push(
                        Todo
                        {
                            id: count.get(),
                            title: title.get(),
                            description: description.get(),
                            is_complete: false,
                        });
                    }
                    > "Add Todo" </button>
                </div>
                <ListDisplay list=list/>
            </div>
        </main>
    }
}

fn main() 
{
    mount_to_body(App);
}

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
pub fn ListDisplay(list_signal: RwSignal<Vec<Todo>>) -> impl IntoView
{
    let (lists, set_lists) = list_signal.split();
    

    view!
    {
        <For
            each=move || lists.get()
            key= |list| list.id
            children = move |list|
            {

                let todo_list = move || lists.get().into_iter().find(|todo| todo.id == list.id);
                view!
                {
                    {move || todo_list().map(|todo|
                    {   
                        let status = move || if todo.is_complete { "Finished" } else {"Unfinished"};
                        view! 
                        {
                            <div class="list">
                                <p>{todo.title.clone()}</p>
                                <p>{todo.description.clone()}</p>
                                <p> {status} </p>
                                <input type="checkbox" on:click= move |_| set_lists.write().iter_mut().find(|t| t.id == todo.id).map(|t| t.is_complete =! t.is_complete).expect("Value not found") />
                            </div>
                        }
                    })}
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

    let (alert, set_alert) = signal(false);

    //this basically creates a Read Write signal and instantly splits it
    //or I can just do signal(Vec::<Todo>::new()); and it'll work lol
    let list_signal = RwSignal::new(Vec::<Todo>::new());
    let (list, set_list) = list_signal.split();


    view!
    {
        <main>
            <div class="container">
                <div class="card">
                    <div class="input-divs">
                        <label for="title-input"> "What shall we do today?" </label>
                        <input id="title-input" type="text" bind:value=(title, set_title)/>
                    </div>
                    <div class="input-divs">
                        <label for="description-input"> "Add more details to it fr" </label>
                        <input id="description-input" type="text" bind:value=(description, set_description)/>
                    </div>
                    {move || alert.get().then(|| view! { <label style="color: red;">"All fields must be filled!"</label> })}

                    <div> 
                        <button type="button"
                        on:click=move |_| 
                        {
                            if title.get().is_empty() || description.get().is_empty()
                            {
                                set_alert.set(true);
                                return;
                            }

                            set_count.set(count.get() + 1);
                            set_list.write().push(
                            Todo
                            {
                                id: count.get(),
                                title: title.get(),
                                description: description.get(),
                                is_complete: false,
                            });
                            
                            set_title.set(String::from(""));
                            set_description.set(String::from(""));
                            set_alert.set(false);
                        }
                        > "Add Todo" </button>
                    </div>
                </div>

                <div class="list-container">
                    <ListDisplay list_signal=list_signal/>
                </div>

            </div>
        </main>
    }
}

fn main() 
{
    _ = console_error_panic_hook::set_once();
    mount_to_body(App);
}

use leptos::prelude::*;
use leptos::mount::mount_to_body;


#[derive(Debug, Clone, Default)]
pub struct Budget
{
    id: u64,
    title: String,
    value: f64, 
}

#[component]
fn App() -> impl IntoView
{
    let (title, set_title) = signal(String::new());
    let (amount, set_amount) = signal(0.0);
    let (budget, set_budget) = signal(Vec::<Budget>::new());
    let (counter, set_counter) = signal(0);

    let total_budget += budget.get();
    view!
    {
        <main>
            <header id="header-fr">
                <h1>"Budget Tracker"</h1>
            </header>

            <div id="main-body">
                <section id="input-body" >
                    <div id="input-card">
                        <label for="title-input"> "Title" </label>
                        <input id="title-input" type="text" on:input=move |event| 
                        {
                            set_title.set(event_target_value(&event));
                        }/>
                        
                        <label for="amount-input"> "Amount" </label>
                        <input id="amount-input" type="text" on:input=move |event| 
                        {
                            let val = event_target_value(&event).parse::<f64>();
                            set_amount.set(val.expect("Should be a valid value fr"));
                        }/>
                        <button style="margin: 4px;" on:click=move |_| 
                        {
                            set_counter.set(counter.get() + 1);
                            set_budget.write().push(Budget
                            {
                                id: counter.get(),
                                title: title.get(),
                                value: amount.get(),
                            });
                        }>
                            "Add Budget"
                        </button>
                    </div>
                </section>

                <section id="money-display">
                    <h1> "Budget Listed" </h1>
                    <For
                        each = move || budget.get()
                        key = |budget| budget.id
                        children = move |budget|
                        {
                            
                            view!
                            {
                                <div class="block">
                                    <p> {budget.title} </p>
                                    <p>"₱"{budget.value} </p>
                                </div>
                            }
                        }
                    />
                </section>
            </div>
        </main>
    }
}
fn main() 
{
    _ = console_error_panic_hook::set_once();
    mount_to_body(App);
}

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
fn ListDisplay(signal: RwSignal<Vec<Budget>>) -> impl IntoView
{
    let (budget, set_budget) = signal.split();
    view!
    {
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
                        <button on:click=move |_|
                        {
                            set_budget.update(|val| val.retain(|x| x.id != budget.id));
                        }
                        > "Remove"
                        </button>
                    </div>
                }
            }
        />
    }
}

#[component]
fn App() -> impl IntoView
{
    let (title, set_title) = signal(String::new());
    let (amount, set_amount) = signal(0.0);

    let rw_signal = RwSignal::new(Vec::<Budget>::new());
    let (budget, set_budget) = rw_signal.split();

    let (counter, set_counter) = signal(0);

    let total_amount = Memo::new(move |_|
    {
        budget.get().iter().map(|x| x.value).sum::<f64>()
    });

    view!
    {
        <main>
            <div id="main-body">
                <section id="input-body" >
                    <div id="input-card">
                        <div id="merchant-image">
                            <img src="https://upload.wikimedia.org/wikipedia/en/1/1d/The_Happy_Merchant.jpg" alt="a very happy merchant" />
                        </div>

                        <label class="input-label" for="title-input"> "Title" </label>
                        <input class="input-box" type="text" on:input=move |event|
                        {
                            set_title.set(event_target_value(&event));
                        }/>
                        
                        <label class="input-label" for="amount-input"> "Amount" </label>
                        <input class="input-box" type="text" on:input=move |event|
                        {
                            let val = event_target_value(&event).parse::<f64>();
                            set_amount.set(val.expect("Should be a valid value fr"));
                        }/>
                        <button id="budget-btn" on:click=move |_| 
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

                    <ListDisplay signal=rw_signal/>

                    <div id="total-banner">
                        <h2> "Total: ₱"{move || total_amount.get().abs()}</h2>
                    </div>

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

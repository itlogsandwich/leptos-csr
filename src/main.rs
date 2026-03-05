use leptos::prelude::*;
use leptos::mount::mount_to_body;


#[derive(Debug, Clone, Default)]
pub struct Money
{
    id: u64,
    value: f64, 
}

#[component]
fn App() -> impl IntoView
{
    let (money, set_money) = signal(0.0);
    let (tracker, set_tracker) = signal(Vec::<Money>::new());
    let (counter, set_counter) = signal(0);

    view!
    {
        <main>
            <header>
                "Financial Tracker"
            </header>
            <section id="input-body" >
                <input type ="numeric" on:input=move |event| 
                {
                    let val = event_target_value(&event).parse::<f64>();
                    set_money.set(val.expect("Should be money fr"));
                }/>

                <button on:click=move |_| 
                {
                    set_counter.set(counter.get() + 1);
                    set_tracker.write().push(Money
                    {
                        id: counter.get(),
                        value: money.get(),
                    });
                }>
                    "Add Money"
                </button>
            </section>
            <section id="money-display">
                <For
                    each = move || tracker.get()
                    key = |tracker| tracker.id
                    children = move |tracker|
                    {
                        view!
                        {
                            <div> 
                                "₱"{tracker.value}
                            </div>
                        }
                    }
                />
            </section>
        </main>
    }
}
fn main() 
{
    _ = console_error_panic_hook::set_once();
    mount_to_body(App);
}

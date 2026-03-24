use leptos::mount::mount_to_body;
use leptos::prelude::*;
use product::Product;

mod product;

#[component]
fn App() -> impl IntoView
{
    let (name,  set_name) = signal(String::from(""));
    let (description, set_description) = signal(String::from(""));
    let (price, set_price) = signal(0.0);

    let product_signal = RwSignal::new(Vec::<Product>::new());
    
    view!
    {
        <main>
            <div class="main-container">
                <nav id="nav-bar">
                
                </nav> 

                <div class="card">
                    <form>
                        <label for="name-input">"Name"</label>
                        <input class="input" name="name-input" type="text" bind:value=(name, set_name)/>

                        <label for="description-input"> "Description" </label>
                        <input class="input" name="description-input" type="text" bind:value=(description, set_description)/>

                        <label for="price-input"> "Price" </label>
                        <input class="input" name="price-input" type="number" on:input=move |e| 
                        {
                            let val = event_target_value(&e)
                                .parse::<f64>()
                                .unwrap_or(0.0);
                            set_price.set(val);
                        }
                        />
                    </form>
                </div>
            </div>                
        </main>
    }
}

fn main() 
{
    mount_to_body(App);
}

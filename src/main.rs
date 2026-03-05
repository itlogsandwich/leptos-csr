use leptos::prelude::*;
use leptos::mount::mount_to_body;

#[component]
fn App() -> impl IntoView
{
    let (count, set_count) = signal(0);
    view!
    {
        <div class="btn-ctn">
            <h1> "Hello, World! - Leptos Running fr" </h1>
            <button 
                on:click= move |_| set_count.update(|x| *x += 1)
                class:btn=move || count.get() % 2 == 0
            > 
                "Click this button to it move fr: "
                {count}
            </button>
            <progress 
                max="100"
                value=move || count.get()
            />
        </div> 
        
        <div class="p-ctn">
            <label class=("p-test", move || (count.get() * 2) % 10 == 0)>
                "Beep Boop doubled: "
                {move || count.get() * 2}
            </label>
            <progress 
                max="200"
                value=move || count.get() * 2
            />
        </div>
    }

}
fn main() 
{
    _ = console_error_panic_hook::set_once();
    mount_to_body(App);
}

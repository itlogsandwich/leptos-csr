use leptos::prelude::*;
use leptos::mount::mount_to_body;

#[component]
fn App() -> impl IntoView
{
    let (count, set_count) = signal(0);
    view!
    {
        <h1> "Hello, World! - Leptos Running fr" </h1>
        <button on:click= move |_| set_count.update(|x| *x += 1)
        class:btn=move || count.get() % 2 == 0
        > 
            "Click this button to see the number go up fr: "
            {count}
        </button>

        <p class=("p-test", move || (count.get() * 2) % 10 == 0)>
            "Beep Boop doubled: "
            {move || count.get() * 2}
        </p>
    }

}
fn main() 
{
    _ = console_error_panic_hook::set_once();
    mount_to_body(App);
}

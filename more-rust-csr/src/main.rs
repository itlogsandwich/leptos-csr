use leptos::prelude::*;

#[component]
fn SideBar() -> impl IntoView
{
    view!
    {
        <div style="height: 100%; background-color: red;">
            <h1> "This is the sidebar" </h1>
        </div>
    }
}

#[component]
fn App() -> impl IntoView
{
    let signal = RwSignal::<i32>::new(0);
    let (count,set_count) = signal.split();

    let container_style = "display: flex; height:100vh; background-color: green;";

    view! 
    {
        <main>
            <div style=container_style>
                <SideBar />
                <div style="width:100vw;">
                    <h1> "Hello Motherfuckers!" </h1> 
                    <div style="display: flex; flex-direction: column;">
                        <button on:click=move |_| set_count.update(|x| *x +=1 ) > 
                            "Click me you fucking cunt"
                            <h1> {{count}} </h1>
                        </button>
                    </div>
                </div>
            </div>
        </main>
    }
}

fn main() {
    leptos::mount::mount_to_body(App);
}

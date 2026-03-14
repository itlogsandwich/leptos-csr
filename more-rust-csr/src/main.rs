use leptos::prelude::*;
use std::time::Duration;

#[component]
fn App() -> impl IntoView
{
    let timer_signal = RwSignal::<Option<IntervalHandle>>::new(None);
    let (timer, set_timer) = timer_signal.split();

    let count_signal = RwSignal::<i32>::new(0);
    let (count,set_count) = count_signal.split();
    
    let one_second = Duration::new(1, 0);
    view! 
    {
        <main>
            <div id="container">
                <div>
                    <div class="card">
                        <h1> {{count}} </h1>
                        <button class="btn" on:click=move |_| 
                        {
                            if let Some(handler) = timer.get()
                            {
                                handler.clear();
                                set_timer.set(None);
                            }
                            else
                            { 
                                if let Ok(h) = set_interval_with_handle(move ||set_count.update(|x| *x +=1 ), one_second)
                                {
                                    set_timer.set(Some(h));
                                }
                
                            } 
                        }
                        > 
                            "Start"
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

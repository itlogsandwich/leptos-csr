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

    let time_display = move || 
    {
        let secs = count.get();
        let hour = secs / 3600;
        let minute = (secs % 3600) / 60;
        let second = secs % 60;
            
        format!("{:02}:{:02}:{:02}", hour,minute,second)
    };

    view! 
    {
        <main>
            <div id="container">
                <div>
                    <div class="card">
                        <div id="clock">
                            "THIS IS A CLOCK"
                        </div>
                        <h1> {{time_display}} </h1>
                        <div
                            style="display: flex; gap: 10px;"
                        >
                            <button class="btn">
                                "Stop"
                            </button>
                            <button class="btn" on:click=move |_| 
                            {
                                if let Some(handler) = timer.get()
                                {
                                    println!("Timer Stopped!");
                                    handler.clear();
                                    set_timer.set(None);

                                }
                                else
                                { 
                                    set_count.set(0);
                                    if let Ok(h) = set_interval_with_handle(move ||set_count.update(|x| *x +=1 ), one_second)
                                    {
                                        println!("Timer Starting...!");
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
            </div>
        </main>
    }
}

fn main() {
    leptos::mount::mount_to_body(App);
}

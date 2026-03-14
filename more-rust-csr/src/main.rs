use leptos::prelude::*;
use std::time::Duration;

#[component]
fn App() -> impl IntoView
{
    let timer_signal = RwSignal::<Option<IntervalHandle>>::new(None);
    let (timer, set_timer) = timer_signal.split();

    let count_signal = RwSignal::<i32>::new(1500);
    let (count,set_count) = count_signal.split();
   
    let one_second = Duration::new(1, 0);

    let clock_offset = move ||
    {
        let offset = (1500.0 - count.get() as f64) / 1500.0 * 251.0;
        offset 
    };

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
                        <h1> "Pomodoro Clock It" </h1>
                        <div id="clock">
                            <svg viewBox="0 0 100 100">
                                <circle cx="50" cy="50" r="40" stroke="black" stroke-width="4" stroke-dasharray="250" stroke-dashoffset=move|| clock_offset().to_string() fill="none"/>
                            </svg>
                        </div>
                        <h1> {{time_display}} </h1>
                        <div
                            style="display: flex; gap: 10px;"
                        >
                            <button class="btn" on:click=move |_|
                            {    
                                if let Some(handler) = timer.get()
                                {
                                    handler.clear();
                                    set_timer.set(None);
                                    set_count.set(1500);
                                }
                                else
                                {
                                    set_count.set(1500);
                                }
                            }>
                                "Reset"

                            </button>
                            <button class="btn" on:click=move |_| 
                            {
                                if let Some(handler) = timer.get()
                                {
                                    handler.clear();
                                    set_timer.set(None);

                                }
                                else
                                { 
                                    if let Ok(h) = set_interval_with_handle(move ||set_count.update(|x| *x -=1 ), one_second)
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
            </div>
        </main>
    }
}

fn main() {
    leptos::mount::mount_to_body(App);
}

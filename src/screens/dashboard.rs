use leptos::*;
use leptos::ev::KeyboardEvent;

/// Renders the home page of your application.
#[component]
pub fn HomePage(cx: Scope) -> impl IntoView {
    // Creates a reactive value to update the button
    let (count, set_count) = create_signal(cx, 0);

    let initial_statement_list = vec!["Nothing here".to_string()];
    let (statement_list, set_statement_list) = create_signal(cx, initial_statement_list);

    let (statement, set_statement) = create_signal(cx, "".to_string());

    let handle_statement = move |ev: KeyboardEvent| {
        // event_target_value is a Leptos helper function
        // it functions the same way as event.target.value
        // in JavaScript, but smooths out some of the typecasting
        // necessary to make this work in Rust
        if ev.key() == "Enter" {
            set_statement_list.update(|s| s.push(event_target_value(&ev)));
            set_statement.set("".to_string());
        }
    };

    view! { cx,
       <div class="h-screen">
            <div class="flex flex-col h-screen">
                <div class="flex px-[20%] flex-col py-5 w-screen grow">
                    <div class="flex flex-col justify-end grow">
                        {move || statement_list.get().iter().map(|s|view!{cx, <p>{s}</p>}).collect::<Vec<_>>()}
                    </div>
                </div>
                <div class="px-[20%] py-5 w-screen flex-none">
                    <input type="text" placeholder="Type here" class="input input-bordered w-full" prop:value=statement on:keypress=handle_statement></input>
                </div>
            </div>
        </div>
    }
}


use leptos::*;
use leptos::ev::KeyboardEvent;

#[component]
pub fn HackerNewsPlugin(cx: Scope) -> impl IntoView {
    let (count, set_count) = create_signal(cx, 0);

    let initial_statement_list = vec!["Nothing here".to_string()];
    let (statement_list, set_statement_list) = create_signal(cx, initial_statement_list);

    let (statement, set_statement) = create_signal(cx, "".to_string());

    let handle_statement = move |ev: KeyboardEvent| {
        if ev.key() == "Enter" {
            set_statement_list.update(|s| s.push(event_target_value(&ev)));
            set_statement.set("".to_string());
        }
    };

    view! { cx,
        <div class="flex px-[20%] flex-col py-5 w-screen grow">
            <div class="flex flex-col justify-end grow">
                {move || statement_list.get().iter().map(|s|view!{cx, <p>{s}</p>}).collect::<Vec<_>>()}
            </div>
        </div>
    }
}


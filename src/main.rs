use std::rc::{Rc, Weak};

use leptos::*;

pub fn main() {
    _ = console_log::init_with_level(log::Level::Debug);
    console_error_panic_hook::set_once();
    mount_to_body(|| {
        view! {
            <App />
        }
    })
}

#[component]
pub fn App() -> impl IntoView {
    let opened = create_rw_signal(true);
    let rc_weak = create_rw_signal::<Option<Weak<i32>>>(None);
    let messages = create_rw_signal(vec!["Press button to destroy SubComponent".to_string()]);

    let handle_click = move |_| {
        opened.set(!opened.get());

        if !opened.get_untracked() {
            leptos::set_timeout(
                move || {
                    let rc_count = rc_weak
                        .get_untracked()
                        .map(|v| v.strong_count())
                        .unwrap_or(0);
                    messages.update(|messages| {
                        messages.push(format!("Done, Rc count: {}", rc_count));
                    });
                },
                std::time::Duration::from_secs(1),
            );
        }
    };

    view! {
        <div>
            <button on:click=handle_click>"Destroy SubComponent"</button>
            <div>
                <pre>{move || messages.get().join("\n")}</pre>
            </div>
            <Show when=move || opened.get()>
                <SubComponent rc_weak=rc_weak messages=messages />
            </Show>
        </div>
    }
}

#[component]
pub fn SubComponent(
    rc_weak: RwSignal<Option<Weak<i32>>>,
    messages: RwSignal<Vec<String>>,
) -> impl IntoView {
    let context_value = Rc::new(1);

    let value = Rc::downgrade(&context_value);
    rc_weak.set(Some(value));

    on_cleanup(move || {
        log::info!("SubComponent cleanup");
        messages.update(|messages| {
            messages.push("SubComponent cleanup".to_string());
        });
    });

    view! {
        <Provider value=context_value>
            <p>"Hello SubComponent"</p>
        </Provider>
    }
}

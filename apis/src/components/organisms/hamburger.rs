use crate::providers::auth_context::AuthContext;
use leptos::html::Div;
use leptos::*;
use leptos_icons::*;
use leptos_use::on_click_outside;

#[component]
pub fn Hamburger(hamburger_show: RwSignal<bool>, children: ChildrenFn) -> impl IntoView {
    let target = create_node_ref::<Div>();
    let _ = on_click_outside(target, move |_| hamburger_show.update(|b| *b = false));
    let children = store_value(children);
    let auth_context = expect_context::<AuthContext>();
    let username = move || {
        if let Some(Ok(Some(user))) = (auth_context.user)() {
            user.username.into_view()
        } else {
            view! { <Icon icon=icondata::ChMenuHamburger/> }
        }
    };
    view! {
        <div node_ref=target class="inline-block mr-10">
            <button
                on:click=move |_| hamburger_show.update(|b| *b = !*b)
                class="bg-ant-blue text-white rounded-md px-2 py-1 m-2 hover:bg-pillbug-teal transform transition-transform duration-300 active:scale-95 whitespace-nowrap"
            >
                {username}
            </button>
            <Show when=hamburger_show>
                <div class="flex flex-col items-stretch absolute bg-even-light dark:bg-even-dark text-black border border-gray-300 rounded-md right-[50px] p-2">
                    {children.with_value(|children| children())}
                </div>
            </Show>
        </div>
    }
}

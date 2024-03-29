use crate::components::atoms::next_game_button::NextGameButton;
use crate::components::molecules::ping::Ping;
use crate::components::organisms::{
    darkmode_toggle::DarkModeToggle, hamburger::Hamburger, logout::Logout,
};
use crate::providers::auth_context::*;

use leptos::logging::log;
use leptos::*;
use leptos_router::use_location;
use shared_types::time_mode::TimeMode;

#[derive(Clone)]
pub struct Redirect(pub RwSignal<String>);

#[component]
pub fn Header(#[prop(optional)] extend_tw_classes: &'static str) -> impl IntoView {
    let auth_context = expect_context::<AuthContext>();
    let hamburger_show = create_rw_signal(false);
    let onclick = move || hamburger_show.update(|b| *b = false);
    view! {
        <header class=format!(
            "w-full fixed top-0 flex justify-between items-center bg-gray-300 dark:bg-gray-700 z-50 max-w-[100vw] {extend_tw_classes}",
        )>
            <a class="ml-10" href="/">
                Home
            </a>
            <Transition>
                {move || {
                    let user = move || match (auth_context.user)() {
                        Some(Ok(Some(user))) => Some(user),
                        _ => None,
                    };
                    view! {
                        <Show
                            when=move || user().is_some()
                            fallback=|| {
                                let hamburger_show = create_rw_signal(false);
                                let onclick = move || hamburger_show.update(|b| *b = false);
                                view! {
                                    <div class="flex items-center">
                                        <a
                                            class="bg-ant-blue hover:bg-pillbug-teal transform transition-transform duration-300 active:scale-95 text-white font-bold py-1 m-1 px-4 rounded"
                                            href="/login"
                                            on:focus=move |_| set_redirect()
                                            on:click=move |_| onclick()
                                        >

                                            Login
                                        </a>
                                        <Hamburger hamburger_show=hamburger_show>
                                            <a
                                                class="bg-ant-blue hover:bg-pillbug-teal transform transition-transform duration-300 active:scale-95 text-white font-bold py-2 px-4 m-1 rounded"
                                                href="/register"
                                                on:focus=move |_| set_redirect()
                                                on:click=move |_| onclick()
                                            >
                                                Register
                                            </a>
                                            <DarkModeToggle on:submit=move |_| onclick()/>
                                            <Ping/>
                                        </Hamburger>
                                    </div>
                                }
                            }
                        >

                            <div class="flex items-center">
                                <NextGameButton time_mode=store_value(TimeMode::RealTime)/>
                                <NextGameButton time_mode=store_value(TimeMode::Correspondence)/>
                                <NextGameButton time_mode=store_value(TimeMode::Untimed)/>
                            </div>

                            <Hamburger hamburger_show=hamburger_show>
                                <a
                                    class="bg-ant-blue hover:bg-pillbug-teal transform transition-transform duration-300 active:scale-95 text-white font-bold py-2 px-4 m-1 rounded"
                                    href=format!("/@/{}", user().expect("User is some").username)

                                    on:click=move |_| onclick()
                                >
                                    Profile
                                </a>
                                <a
                                    class="bg-ant-blue hover:bg-pillbug-teal transform transition-transform duration-300 active:scale-95 text-white font-bold py-2 px-4 m-1 rounded"
                                    href="/account"
                                    on:focus=move |_| set_redirect()
                                    on:click=move |_| onclick()
                                >
                                    Edit Account
                                </a>
                                <a
                                    class="bg-ant-blue hover:bg-pillbug-teal transform transition-transform duration-300 active:scale-95 text-white font-bold py-2 px-4 m-1 rounded"
                                    href="/config"
                                    on:focus=move |_| set_redirect()
                                    on:click=move |_| onclick()
                                >
                                    Config
                                </a>
                                <DarkModeToggle on:submit=move |_| onclick()/>
                                <Logout on:submit=move |_| onclick()/>
                                <Ping/>
                            </Hamburger>
                        </Show>
                    }
                }}

            </Transition>
        </header>
    }
}

fn set_redirect() {
    let referrer = RwSignal::new(String::from("/"));
    let location = use_location().pathname.get();
    log!("We have location");
    referrer.update(|s| *s = location);
    log!("Referrer is {}", referrer.get_untracked());
    provide_context(Redirect(referrer));
}

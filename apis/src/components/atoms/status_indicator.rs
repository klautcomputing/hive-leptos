use crate::{common::server_result::UserStatus, providers::online_users::OnlineUsersSignal};
use leptos::*;
use leptos_icons::{BiIcon::BiCircleRegular, BiIcon::BiCircleSolid, Icon};

#[component]
pub fn StatusIndicator(username: String) -> impl IntoView {
    let online_users = expect_context::<OnlineUsersSignal>();
    let display_icon = move || match (online_users.signal)().username_status.get(&username) {
        Some(UserStatus::Online) => {
            view! { <Icon icon=Icon::from(BiCircleSolid) class="fill-green-500"/> }
        }
        Some(UserStatus::Away) => view! { <p>Away icon</p> }.into_view(),
        _ => view! { <Icon icon=Icon::from(BiCircleRegular)/> },
    };
    display_icon
}

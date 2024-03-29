use hive_lib::{color::Color, game_status::GameStatus};
use leptos::*;

use crate::{
    components::{
        atoms::{profile_link::ProfileLink, status_indicator::StatusIndicator},
        molecules::rating_and_change::RatingAndChangeDynamic,
    },
    providers::game_state::GameStateSignal,
};

#[component]
pub fn UserWithRating(
    side: Color,
    #[prop(optional)] text_color: &'static str,
    #[prop(optional)] is_tall: Signal<bool>,
) -> impl IntoView {
    let game_state = expect_context::<GameStateSignal>();
    let player = move || match side {
        Color::White => game_state
            .signal
            .get()
            .game_response
            .map(|g| g.white_player),
        Color::Black => game_state
            .signal
            .get()
            .game_response
            .map(|g| g.black_player),
    };
    let is_finished = create_memo(move |_| {
        matches!(
            (game_state.signal)().state.game_status,
            GameStatus::Finished(_)
        )
    });
    let username = move || player().map_or(String::new(), |p| p.username);
    let rating = move || player().map_or(String::new(), |p| p.rating.to_string());
    view! {
        <div class=move || {
            format!(
                "ml-1 flex items-center {} justify-center",
                if is_tall() { "flex-row gap-1" } else { "flex-col" },
            )
        }>
            {move || {
                view! {
                    <div class="flex items-center">
                        <StatusIndicator username=username()/>
                        <ProfileLink username=username() extend_tw_classes=text_color/>
                    </div>
                }
            }}
            <Show
                when=is_finished
                fallback=move || {
                    view! { <div class=format!("{text_color}")>{rating()}</div> }
                }
            >

                <RatingAndChangeDynamic extend_tw_classes=text_color side=side/>
            </Show>
        </div>
    }
}

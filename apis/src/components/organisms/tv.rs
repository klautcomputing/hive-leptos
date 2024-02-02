use std::str::FromStr;

use crate::{
    components::{
        atoms::svgs::Svgs,
        molecules::{thumbnail_pieces::ThumbnailPieces, time_row::TimeRow},
    },
    providers::games::GamesSignal,
};
use leptos::*;
use shared_types::time_mode::TimeMode;

#[component]
pub fn Tv() -> impl IntoView {
    let games = expect_context::<GamesSignal>();
    let live_games = move || (games.live)().live_games;

    view! {
        <div class="flex flex-col items-center md:pt-12">
            <div class="flex flex-col md:flex-row gap-1 items-center flex-wrap w-full">
                <For each=live_games key=|(k, v)| (k.to_owned(), v.turn) let:game>
                    <div class="h-60 w-60 mx-2 relative dark:odd:bg-odd-dark dark:even:bg-even-dark  odd:bg-odd-light even:bg-even-light flex flex-col items-center">
                        <div class="flex flex-col items-center">
                            {format!(
                                "{} {} vs {} {}",
                                game.1.white_player.username,
                                game.1.white_player.rating,
                                game.1.black_player.username,
                                game.1.black_player.rating,
                            )}
                            <div class="flex items-center">
                                {if game.1.rated { "RATED " } else { "CASUAL " }}
                                <TimeRow
                                    time_mode=TimeMode::from_str(&game.1.time_mode)
                                        .expect("Valid time mode")
                                    time_base=game.1.time_base
                                    increment=game.1.time_increment
                                />
                            </div>
                        </div>
                        <svg
                            viewBox="1100 500 400 510"
                            class="touch-none h-full w-full"
                            xmlns="http://www.w3.org/2000/svg"
                        >
                            <Svgs/>
                            <g>
                                <ThumbnailPieces game=game.1/>
                            </g>
                        </svg>
                        <a
                            class="h-full w-full absolute top-0 left-0 z-10"
                            href=format!("/game/{}", game.0)
                        ></a>
                    </div>
                </For>
            </div>
        </div>
    }
}
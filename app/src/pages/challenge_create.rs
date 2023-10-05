use crate::functions::challenges::create::CreateChallenge;
use leptos::*;
use leptos_router::ActionForm;

#[component]
pub fn ChallengeCreate() -> impl IntoView {
    let create_challenge_action = create_server_action::<CreateChallenge>();
    view! {
        <ActionForm action=create_challenge_action>
            <div>
                <div>
                    <select name="public">
                        <option value="true">"Public"</option>
                        <option value="false">"Private"</option>
                    </select>
                </div>
                <div>
                    <select name="rated">
                        <option value="true">"Rated"</option>
                        <option value="false">"Unrated"</option>
                    </select>
                </div>
                <div>
                    <select name="tournament_queen_rule">
                        <option value="true">"Tournament rules"</option>
                        <option value="false">"Queen first"</option>
                    </select>
                </div>
            </div>

            <select name="color_choice">
                <option value="Random">"Random"</option>
                <option value="White">"White"</option>
                <option value="Black">"Black"</option>
            </select>
            <select name="game_type">
                <option value="MLP">"PLM"</option>
                <option value="Base">"Base"</option>
            </select>
            <input
                type="submit"
                class="bg-blue-500 hover:bg-blue-700 text-white font-bold py-2 px-4 rounded focus:outline-none focus:shadow-outline"
                value="Create new challenge"
            />
        </ActionForm>
    }
}

mod components;
mod library;
use components::square::Square;
use library::{board::BoardState, player::PlayerColor};
use yew::prelude::*;
use yewdux::prelude::*;

#[function_component]
fn Board() -> Html {
    let (board_state, dispatch) = use_store::<BoardState>();
    let board_size = board_state.points.len();
    let list = 0..board_size;
    let color = match board_state.turn {
        PlayerColor::WHITE => "White",
        PlayerColor::BLACK => "Black",
    };
    html! {
        <div class={"flex flex-col h-[100vh] justify-center items-center bg-[#5A5A5A]"}>
            <p class={"text-white"}>{color}{"'s turn"}</p>
            <div class={"flex flex-row flex-wrap justify-center items-center self-center h-[50rem] w-[50rem]"}>
                {list.into_iter().map(|i| html! {<Square idx={i}/>}).rev().collect::<Html>()}
            </div>
        </div>
    }
}

fn main() {
    yew::Renderer::<Board>::new().render();
}

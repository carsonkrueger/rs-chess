mod components;
mod library;
use components::square::Square;
use library::board::BoardState;
use yew::prelude::*;
use yewdux::prelude::*;

#[function_component]
fn Board() -> Html {
    let (board_state, dispatch) = use_store::<BoardState>();
    let board_size = board_state.points.len();
    let list = (0..board_size);
    html! {
        <div class={"flex flex-row flex-wrap justify-center items-center h-[50rem] w-[50rem]"}>
            <img class={"absolute z-0 h-[50rem] w-[50rem] object-contain"} src={"img/chessboard.png"}/>
            {list.into_iter().map(|i| html! {<Square idx={i}/>}).rev().collect::<Html>()}
        </div>
    }
}

fn main() {
    yew::Renderer::<Board>::new().render();
}

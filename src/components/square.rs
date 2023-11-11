use crate::library::board::BoardState;
use yew::prelude::*;
use yewdux::prelude::use_store;

#[derive(Properties, PartialEq)]
pub struct Props {
    pub idx: usize,
}

#[function_component]
pub fn Square(props: &Props) -> Html {
    let (board_state, dispatch) = use_store::<BoardState>();
    let point = &board_state.points[props.idx];
    let is_piece = point.data.is_some();
    html! {
        <div class={"z-10 h-[6.25rem] w-[6.25rem]"}>
            if is_piece {<img class={"object-fill"} src={point.data.unwrap().img_path()} />}
        </div>
    }
}

use std::rc::Rc;

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
    let on_click: Callback<BoardState> = dispatch.reduce_callback(|s| {
        s.select1_idx = Some(0);
        s
    });
    let point = &board_state.points[props.idx];
    let is_piece = point.data.is_some();
    html! {
        <div class={"focus:cursor-grabbing z-10 h-[6.25rem] w-[6.25rem]"}>
            if is_piece {<img class={"object-fill"} src={point.data.unwrap().img_path()} />}
        </div>
    }
}

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
    let idx = props.idx.clone();
    let onclick: Callback<MouseEvent> = dispatch.reduce_mut_callback(move |board_state| {
        println!("clicked piece");
        if board_state.select1_idx.is_some() && board_state.select2_idx.is_some() {
            let i1 = board_state.select1_idx.unwrap();
            let i2 = board_state.select2_idx.unwrap();
            if i1 != i2 {
                board_state.play().expect("error moving piece");
            }
            board_state.select1_idx = None;
            board_state.select2_idx = None;
        } else if board_state.select1_idx.is_none() {
            println!("selecting 1 idx: {}", idx);
            board_state.select1_idx = Some(idx);
        } else if board_state.select2_idx.is_none() {
            println!("selecting 2 idx: {}", idx);
            board_state.select2_idx = Some(idx);
        }
    });
    let point = &board_state.points[props.idx];
    let is_piece = point.is_some();
    html! {
        <div class={"z-10 h-[6.25rem] w-[6.25rem]"}>
            if is_piece {<img onclick={onclick}  class={"hover:cursor-grabbing object-fill"} src={point.unwrap().img_path()} />}
        </div>
    }
}

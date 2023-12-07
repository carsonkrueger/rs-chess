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

    let onclick: Callback<MouseEvent> = dispatch.reduce_mut_callback(move |s: &mut BoardState| {
        s.select_then_play(idx);
        // if s.select1_idx.is_none() {
        //     s.select1_idx = Some(idx);
        // } else if s.select2_idx.is_none() {
        //     s.select2_idx = Some(idx);
        // }

        // if s.select1_idx.is_some() && s.select2_idx.is_some() {
        //     let i1 = s.select1_idx.unwrap();
        //     let i2 = s.select2_idx.unwrap();
        //     if i1 != i2 {
        //         s.play();
        //     }
        //     s.select1_idx = None;
        //     s.select2_idx = None;
        // }
    });

    let point = &board_state.points[props.idx];
    let is_piece = point.is_some();
    html! {
        <div onclick={onclick} class={"z-10 h-[6.25rem] w-[6.25rem]"}>
            if is_piece {<img class={"hover:cursor-grabbing object-fill"} src={point.unwrap().img_path()} />}
        </div>
    }
}

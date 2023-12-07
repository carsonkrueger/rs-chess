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
        s.select(idx);
    });

    let point = &board_state.points[props.idx];
    let is_piece = point.is_some();

    html! {
        <div onclick={onclick} class={format!("z-10 h-[6.25rem] w-[6.25rem] {} {}", bg_color(idx), rounded(idx))}>
            if is_piece {<img class={"hover:cursor-grabbing object-fill"} src={point.unwrap().img_path()} />}
            if board_state.select1_idx.is_some() && board_state.select1_idx.unwrap() == idx {<div class={"bg-transparent"}/>}
        </div>
    }
}

fn bg_color(idx: usize) -> String {
    let is_odd_sq = idx % 2 == 1;
    let is_odd_row = (idx / 8) % 2 == 1;

    match is_odd_row {
        true => match is_odd_sq {
            true => String::from("bg-white"),
            false => String::from("bg-[#3e9c57]"),
        },
        false => match !is_odd_sq {
            true => String::from("bg-white"),
            false => String::from("bg-[#3e9c57]"),
        },
    }
}

fn rounded(idx: usize) -> String {
    match idx {
        0 => String::from("rounded-br-lg"),
        7 => String::from("rounded-bl-lg"),
        63 => String::from("rounded-tl-lg"),
        56 => String::from("rounded-tr-lg"),
        _ => String::new(),
    }
}

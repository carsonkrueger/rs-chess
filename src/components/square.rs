use std::rc::Rc;

use crate::library::board::{Action, BoardState};
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
    // let onclick = dispatch.reduce(move |mut old_state| {
    //     // print!("clicked");
    //     let mut board_state = old_state.clone();
    //     if board_state.select1_idx.is_some() && board_state.select2_idx.is_some() {
    //         let i1 = board_state.select1_idx.unwrap();
    //         let i2 = board_state.select2_idx.unwrap();
    //         if i1 != i2 {
    //             board_state.play().expect("error moving piece");
    //         }
    //         board_state.select1_idx = None;
    //         board_state.select2_idx = None;
    //         panic!("yo");
    //     } else if board_state.select1_idx.is_none() {
    //         // println!("selecting 1 idx: {}", idx);
    //         board_state.select1_idx = Some(idx);
    //     } else if board_state.select2_idx.is_none() {
    //         // println!("selecting 2 idx: {}", idx);
    //         board_state.select2_idx = Some(idx);
    //     }
    //     board_state
    // });

    let onclick = move |_: MouseEvent| {
        dispatch.reduce(move |mut old_state| {
            // print!("clicked");
            let mut new_state = old_state.clone();
            if new_state.select1_idx.is_some() && new_state.select2_idx.is_some() {
                let i1 = new_state.select1_idx.unwrap();
                let i2 = new_state.select2_idx.unwrap();
                if i1 != i2 {
                    new_state.play().expect("error moving piece");
                }
                new_state.select1_idx = None;
                new_state.select2_idx = None;
                panic!("yo");
            } else if new_state.select1_idx.is_none() {
                // println!("selecting 1 idx: {}", idx);
                new_state.select1_idx = Some(idx);
            } else if new_state.select2_idx.is_none() {
                // println!("selecting 2 idx: {}", idx);
                new_state.select2_idx = Some(idx);
            }
            new_state
        });
    };

    // let handle_click = {
    //     let dispatch_clone = dispatch.clone();
    //     move |mut new_state: Rc<BoardState>| {
    //         let mut new_state = board_state.clone();
    //         if new_state.select1_idx.is_some() && new_state.select2_idx.is_some() {
    //             let i1 = new_state.select1_idx.unwrap();
    //             let i2 = new_state.select2_idx.unwrap();
    //             if i1 != i2 {
    //                 new_state.play().expect("error moving piece");
    //             }
    //             new_state.select1_idx = None;
    //             new_state.select2_idx = None;
    //         } else if new_state.select1_idx.is_none() {
    //             // println!("selecting 1 idx: {}", idx);
    //             new_state.select1_idx = Some(idx);
    //         } else if new_state.select2_idx.is_none() {
    //             // println!("selecting 2 idx: {}", idx);
    //             new_state.select2_idx = Some(idx);
    //         }
    //         board_state
    //     }
    // };

    // let onclick = dispatch.reduce_mut_callback(handle_click);

    let point = &board_state.points[props.idx];
    let is_piece = point.is_some();
    html! {
        <div onclick={onclick} class={"z-10 h-[6.25rem] w-[6.25rem]"}>
            if is_piece {<img class={"hover:cursor-grabbing object-fill"} src={point.unwrap().img_path()} />}
        </div>
    }
}

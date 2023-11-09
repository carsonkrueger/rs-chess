use crate::library::{piece::PieceType, point::Point};
use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct Props {
    pub piece: crate::library::piece::Piece,
}

#[function_component]
pub fn Piece(props: &Props) -> Html {
    html! {
        <div class={"h-[6.25rem] w-[6.25rem]"}>
            // <img class={"h-full w-full"} src={format!("img/{}", props.piece.img_path())} />
        </div>
    }
}

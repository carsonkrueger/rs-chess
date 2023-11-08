use crate::library::{piece::ChessPieceType, point::Point};
use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct Props {
    pub piece: ChessPieceType,
    pub point: Point<u8>,
}

#[function_component]
pub fn Piece(props: &Props) -> Html {
    html! {
        <div class={"h-[6.25rem] w-[6.25rem]"}>
            <img class={"h-full w-full"} src={format!("img/{}", props.piece.img_path())} />
        </div>
    }
}

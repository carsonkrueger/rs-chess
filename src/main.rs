mod components;
mod library;
use components::piece::Piece;
use library::{piece::PieceType, point::Point};
use yew::prelude::*;

#[function_component]
fn Board() -> Html {
    html! {
        <div class={"flex justify-center items-center"}>
            <img class={"h-[50rem]"} src={"img/chessboard.png"}/>
            // <Piece piece={PieceType::ROOK1} point={Point::from((0,0))}/>
        </div>
    }
}

fn main() {
    yew::Renderer::<Board>::new().render();
}

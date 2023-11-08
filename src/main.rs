mod components;
mod library;
use components::piece::Piece;
use library::{piece::ChessPieceType, point::Point};
use num_traits::Num;
use yew::prelude::*;

struct PlayerState<T: Num> {
    king_point: Option<Point<T>>,
    queen_point: Option<Point<T>>,
    bishop1_point: Option<Point<T>>,
    bishop2_point: Option<Point<T>>,
    knight1_point: Option<Point<T>>,
    knight2_point: Option<Point<T>>,
    rook1_point: Option<Point<T>>,
    rook2_point: Option<Point<T>>,
    pawn1_point: Option<Point<T>>,
    pawn2_point: Option<Point<T>>,
    pawn3_point: Option<Point<T>>,
    pawn4_point: Option<Point<T>>,
    pawn5_point: Option<Point<T>>,
    pawn6_point: Option<Point<T>>,
    pawn7_point: Option<Point<T>>,
    pawn8_point: Option<Point<T>>,
}

struct BoardState {
    w_player: PlayerState<u8>,
    b_player: PlayerState<u8>,
}

#[function_component]
fn Board() -> Html {
    html! {
        <div class={"flex justify-center items-center"}>
            <img class={"h-[50rem]"} src={"img/chessboard.png"}/>
            <Piece piece={ChessPieceType::WROOK} point={Point::from((0,0))}/>
        </div>
    }
}

fn main() {
    yew::Renderer::<Board>::new().render();
}

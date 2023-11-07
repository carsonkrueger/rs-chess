use crate::library::{images::ChessImgPath, point::Point};
use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct Props {
    pub img_path: ChessImgPath,
    pub point: Point<u8>,
}

#[function_component]
pub fn Piece(props: &Props) -> Html {
    println!("{}", props.img_path.to_string());
    html! {
        <div class={"h-[6.25rem] w-[6.25rem]"}>
            <img class={"h-full w-full"} src={format!("img/{}", props.img_path.to_string())} />
        </div>
    }
}

use yew::prelude::*;

#[function_component(Game)]
fn game() -> Html {
    html! {
        <>
            <img src={"img/chessboard.jpg"}/>
            <div class={classes!("absolute")} />
        </>
    }
}

fn main() {
    yew::Renderer::<Game>::new().render();
}

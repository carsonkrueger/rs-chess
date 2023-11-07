#[derive(PartialEq)]
pub enum ChessImgPath {
    WKING,
    WQUEEN,
    WBISHOP,
    WROOK,
    WPAWN,
    BKING,
    BQUEEN,
    BBISHOP,
    BROOK,
    BPAWN,
}

impl ToString for ChessImgPath {
    fn to_string(&self) -> String {
        match &self {
            ChessImgPath::WKING => String::from("img/w_king.png"),
            ChessImgPath::WQUEEN => String::from("img/w_queen.png"),
            ChessImgPath::WBISHOP => String::from("img/w_bishop.png"),
            ChessImgPath::WROOK => String::from("img/w_rook.png"),
            ChessImgPath::WPAWN => String::from("img/w_pawn.png"),
            ChessImgPath::BKING => String::from("img/b_king.png"),
            ChessImgPath::BQUEEN => String::from("img/b_queen.png"),
            ChessImgPath::BBISHOP => String::from("img/b_bishop.png"),
            ChessImgPath::BROOK => String::from("img/b_rook.png"),
            ChessImgPath::BPAWN => String::from("img/b_pawn.png"),
        }
    }
}

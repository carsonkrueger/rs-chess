use crate::library::board::BoardState;

#[test]
fn in_bounds() {
    assert!(BoardState::in_bounds(0));
    assert!(BoardState::in_bounds(1));
    assert!(BoardState::in_bounds(63));
    assert!(!BoardState::in_bounds(64));
    assert!(!BoardState::in_bounds(150));
}

#[test]
fn row_num() {
    assert!(BoardState::row_num(0) == 0);
    assert!(BoardState::row_num(7) == 0);
    assert!(BoardState::row_num(8) == 1);
    assert!(BoardState::row_num(63) == 7);
}

#[test]
fn col_num() {
    assert!(BoardState::col_num(0) == 0);
    assert!(BoardState::col_num(7) == 7);
    assert!(BoardState::col_num(8) == 0);
    assert!(BoardState::col_num(63) == 7);
}

#[test]
fn is_forward() {
    assert!(BoardState::is_forward(
        0,
        8,
        crate::library::player::PlayerColor::White
    ));
    assert!(BoardState::is_forward(
        36,
        44,
        crate::library::player::PlayerColor::White
    ));
    assert!(BoardState::is_forward(
        60,
        52,
        crate::library::player::PlayerColor::Black
    ));
    assert!(!BoardState::is_forward(
        48,
        32,
        crate::library::player::PlayerColor::Black
    ));
}

#[test]
fn is_adjacent() {
    assert!(BoardState::is_adjacent(9, 8));
    assert!(BoardState::is_adjacent(9, 1));
    assert!(BoardState::is_adjacent(9, 10));
    assert!(BoardState::is_adjacent(9, 17));
    assert!(BoardState::is_adjacent(9, 0));
    assert!(BoardState::is_adjacent(9, 2));
    assert!(BoardState::is_adjacent(9, 16));
    assert!(BoardState::is_adjacent(9, 18));

    assert!(!BoardState::is_adjacent(9, 3));
    assert!(!BoardState::is_adjacent(9, 11));
}

#[test]
fn is_diagnol() {
    assert!(BoardState::is_diagnol(24, 15));
    assert!(BoardState::is_diagnol(24, 6));
    assert!(BoardState::is_diagnol(24, 17));
    assert!(BoardState::is_diagnol(24, 10));
    assert!(BoardState::is_diagnol(24, 31));
    assert!(BoardState::is_diagnol(24, 33));

    assert!(!BoardState::is_diagnol(24, 23))
}

#[test]
fn is_knight_hop() {
    assert!(BoardState::is_knight_hop(17, 0));
    assert!(BoardState::is_knight_hop(17, 2));
    assert!(BoardState::is_knight_hop(17, 11));
    assert!(BoardState::is_knight_hop(17, 27));

    assert!(!BoardState::is_knight_hop(25, 34));
    assert!(!BoardState::is_knight_hop(25, 28));
    assert!(!BoardState::is_knight_hop(25, 36));
}

#[test]
fn is_slide() {
    let b = BoardState::default();
    assert!(b.is_slide(9, 1));
    assert!(b.is_slide(9, 49));
    assert!(b.is_slide(9, 8));
    assert!(b.is_slide(17, 23));
    assert!(b.is_slide(9, 17));

    assert!(!b.is_slide(9, 16));
    assert!(!b.is_slide(9, 18));
}

#[test]
fn are_friendly() {
    let b = BoardState::default();
    assert!(b.are_friendly(0, 1));
    assert!(b.are_friendly(7, 15));

    assert!(!b.are_friendly(7, 16));
    assert!(!b.are_friendly(29, 19));
    assert!(!b.are_friendly(23, 47));

    assert!(b.are_friendly(53, 62));
    assert!(b.are_friendly(50, 55));
}

use sgf_parser::{SgfToken, Action, parse};

pub fn parse(sgf: &str) {
    let tree = parse(&sgf)?;

    let mut moves = Vec::new();
    for node in tree.iter() {
        for token in node.tokens.iter() {
            match token {
                SgfToken::Move { action, .. } => {
                    match action {
                        Action::Move(x, y) => {
                            moves.push((x, y));
                        },
                        _ => {}
                    }
                },
                _ => {}
            }
        }
    }
}

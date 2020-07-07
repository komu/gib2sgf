use crate::gib::{Gib, GibParseError};
use crate::sgf::{SgfCollection, SgfTree, SgfNode};
use crate::go::PlayerColor;

const VERSION: &'static str = env!("CARGO_PKG_VERSION");

///
/// Converts a GIB file to SGF. Both the input and output are represented
/// as in-memory contents of the files.
///
pub fn gib_to_sgf(gib_data: &str) -> Result<String, GibParseError> {
    let gib = Gib::parse(gib_data)?;

    let mut root = SgfNode::new();

    // https://www.red-bean.com/sgf/properties.html
    root.set_property_maybe("PB", gib.get_nick(PlayerColor::Black));
    root.set_property_maybe("BR", gib.get_rank(PlayerColor::Black));
    root.set_property_maybe("PW", gib.get_nick(PlayerColor::White));
    root.set_property_maybe("WR", gib.get_rank(PlayerColor::White));
    root.set_property_maybe("KM", gib.get_komi());
    root.set_property_maybe("DT", gib.get_date());
    root.set_property_maybe("RE", gib.get_result());
    root.set_property_maybe("SO", gib.get_game_place());

    root.set_property("RU", "Japanese"); // Assume files are from Tygem which uses Japanese rules
    root.set_property("SZ", "19"); // Assume board size

    // General metadata
    root.set_property("GM", "1"); // Game, 1 = Go
    root.set_property("FF", "4"); // File format version
    root.set_property("CA", "UTF-8"); // Charset
    root.set_property("AP", format!("gib2sgf:{}", VERSION).as_str()); // Application that generated the file

    if let Some(handicap) = gib.get_handicap() {
        root.set_property("HA", handicap);
        for point in handicap.handicap_points() {
            root.set_property("AB", point)
        }
    }

    let mut game = SgfTree::new();
    game.add_node(root);
    for mv in gib.get_moves() {
        game.add_move(mv.player, mv.coordinate)
    }

    let sgf = SgfCollection::from_game(game);

    Ok(sgf.to_sgf())
}

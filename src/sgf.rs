use linked_hash_map::LinkedHashMap;
use crate::go::{PlayerColor, BoardCoordinate, Score, GameResult, Handicap};
use crate::time::LocalDate;

pub struct SgfCollection {
    trees: Vec<SgfTree>
}

pub struct SgfTree {
    nodes: Vec<SgfNode>,
    children: Vec<SgfTree>,
}

pub struct SgfNode {
    properties: LinkedHashMap<String, Vec<String>>
}

impl SgfCollection {
    pub fn from_game(tree: SgfTree) -> SgfCollection {
        SgfCollection { trees: vec![tree] }
    }

    pub fn to_sgf(&self) -> String {
        let mut result = String::new();

        for tree in &self.trees {
            tree.to_sgf(&mut result);
        }

        result
    }
}

impl SgfTree {
    pub fn new() -> SgfTree {
        SgfTree { nodes: Vec::new(), children: Vec::new() }
    }

    pub fn add_node(&mut self, node: SgfNode) {
        self.nodes.push(node)
    }

    pub fn add_move(&mut self, color: PlayerColor, coordinate: BoardCoordinate) {
        self.nodes.push(SgfNode::new_move(color, coordinate))
    }

    fn to_sgf(&self, result: &mut String) {
        result.push('(');

        for node in &self.nodes {
            result.push(';');

            for (name, values) in &node.properties {
                result.push_str(name);
                for value in values {
                    result.push('[');
                    result.push_str(value);
                    result.push(']');
                }
            }
        }

        for child in &self.children {
            child.to_sgf(result);
        }

        result.push(')');
    }
}

impl SgfNode {
    pub fn new() -> SgfNode {
        SgfNode { properties: LinkedHashMap::new() }
    }

    pub fn new_move(color: PlayerColor, coordinate: BoardCoordinate) -> SgfNode {
        let mut node = SgfNode::new();
        node.set_property(color.pick("B", "W"), coordinate);
        node
    }

    pub fn set_property(&mut self, name: &str, value: impl ToSgf) {
        self.properties.insert(name.to_string(), vec![value.to_sgf()]);
    }

    pub fn set_property_list(&mut self, name: &str, values: Vec<impl ToSgf>) {
        self.properties.insert(name.to_string(), values.iter().map(|v| v.to_sgf()).collect());
    }

    pub fn set_property_maybe(&mut self, name: &str, value: Option<impl ToSgf>) {
        if let Some(v) = value {
            self.set_property(name, v)
        }
    }
}

pub trait ToSgf {
    fn to_sgf(&self) -> String;
}

impl ToSgf for &str {
    fn to_sgf(&self) -> String {
        self.to_string()
    }
}

impl ToSgf for Score {
    fn to_sgf(&self) -> String {
        self.to_string()
    }
}

impl ToSgf for LocalDate {
    fn to_sgf(&self) -> String {
        self.iso_string()
    }
}

impl ToSgf for BoardCoordinate {
    fn to_sgf(&self) -> String {
        let x = (b'a' + self.x) as char;
        let y = (b'a' + self.y) as char;
        format!("{}{}", x, y)
    }
}

impl PlayerColor {
    fn sgf_color(&self) -> &str {
        self.pick("B", "W")
    }
}

impl ToSgf for GameResult {
    fn to_sgf(&self) -> String {
        match self {
            GameResult::Jigo => String::from("0"),
            GameResult::Count(winner, score) =>
                if let Some(score) = score {
                    format!("{}+{}", winner.sgf_color(), score.to_sgf())
                } else {
                    format!("{}+?", winner.sgf_color())
                }
            GameResult::Resign(winner) => format!("{}+R", winner.sgf_color()),
            GameResult::Time(winner) => format!("{}+T", winner.sgf_color()),
            GameResult::Forfeit(winner) => format!("{}+F", winner.sgf_color()),
        }
    }
}

impl ToSgf for Handicap {
    fn to_sgf(&self) -> String {
        self.to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::go::GameResult;

    #[test]
    fn test_output_generation() {
        let mut root = SgfNode::new();

        root.set_property("PB", "Honinbo Shusaku");
        root.set_property("BR", "7P");
        root.set_property("PW", "Go Seigen");
        root.set_property("WR", "9P");

        root.set_property("RE", "B+37.5");
        root.set_property("KM", Score::new(6.5));
        root.set_property("DT", LocalDate::ymd(2020, 2, 29).unwrap());
        root.set_property("FF", "4");
        root.set_property("GM", "1");
        root.set_property("CA", "UTF-8");
        root.set_property("SZ", "19");

        let mut tree = SgfTree::new();
        tree.nodes.push(root);

        let collection = SgfCollection::from_game(tree);

        assert_eq!("(;PB[Honinbo Shusaku]BR[7P]PW[Go Seigen]WR[9P]RE[B+37.5]KM[6.5]DT[2020-02-29]FF[4]GM[1]CA[UTF-8]SZ[19])", collection.to_sgf());
    }

    #[test]
    fn formatting_game_results() {
        assert_eq!(GameResult::Jigo.to_sgf(), "0");
        assert_eq!(GameResult::Count(PlayerColor::Black, Some(Score::new(2.5))).to_sgf(), "B+2.5");
        assert_eq!(GameResult::Count(PlayerColor::White, Some(Score::new(50.0))).to_sgf(), "W+50");
        assert_eq!(GameResult::Count(PlayerColor::White, None).to_sgf(), "W+?");
        assert_eq!(GameResult::Resign(PlayerColor::White).to_sgf(), "W+R");
        assert_eq!(GameResult::Time(PlayerColor::White).to_sgf(), "W+T");
        assert_eq!(GameResult::Forfeit(PlayerColor::White).to_sgf(), "W+F");
    }

    #[test]
    fn board_coordinates() {
        assert_eq!(BoardCoordinate { x: 0, y: 0 }.to_sgf(), "aa");
        assert_eq!(BoardCoordinate { x: 4, y: 8 }.to_sgf(), "ei");
        assert_eq!(BoardCoordinate { x: 18, y: 17 }.to_sgf(), "sr");
    }
}

use crate::{
    core::{CoreError, Result, Solver},
    maths,
    string_scanner::StringScanner,
};
use std::collections::HashMap;

pub fn part_1() -> Box<dyn Solver> {
    Box::<Part1>::default()
}

pub fn part_2() -> Box<dyn Solver> {
    Box::<Part2>::default()
}

#[derive(Debug, Default)]
struct Part1(MapBuilder);

impl Solver for Part1 {
    fn handle_line(&mut self, line: &str) -> Result<()> {
        self.0.add_line(line)?;
        Ok(())
    }

    fn extract_solution(&self) -> Result<String> {
        let map = self.0.build()?;
        let num_steps = map.calculate_distance(NodeId::new("AAA"), &is_zzz);
        Ok(num_steps.to_string())
    }
}

#[derive(Debug, Default)]
struct Part2(MapBuilder);

impl Solver for Part2 {
    fn handle_line(&mut self, line: &str) -> Result<()> {
        self.0.add_line(line)?;
        Ok(())
    }

    fn extract_solution(&self) -> Result<String> {
        let map = self.0.build()?;
        let start_nodes = map.start_nodes();
        let nums: Vec<u64> = start_nodes
            .iter()
            .map(|node_id| map.calculate_distance(node_id.clone(), &ends_with_z))
            .collect();
        let total = maths::lcm(&nums);
        Ok(total.map_or("".to_string(), |n| n.to_string()))
    }
}

fn is_zzz(node_id: &NodeId) -> bool {
    node_id == &NodeId::new("ZZZ")
}

fn ends_with_z(node_id: &NodeId) -> bool {
    node_id.ends_with('Z')
}

#[derive(Debug)]
struct Map {
    directions: Vec<Direction>,
    nodes: HashMap<NodeId, Node>,
}

impl Map {
    fn calculate_distance(&self, start_id: NodeId, end: &dyn Fn(&NodeId) -> bool) -> u64 {
        let mut current_node_id = start_id.clone();
        let mut num_steps = 0;
        let mut directions = self.directions.iter().cycle();

        loop {
            let direction = directions.next().unwrap();

            let node = self.nodes.get(&current_node_id).unwrap();
            current_node_id = node.next_node_id(*direction);

            num_steps += 1;

            if end(&current_node_id) {
                break;
            }
        }

        num_steps
    }

    fn start_nodes(&self) -> Vec<NodeId> {
        self.nodes
            .keys()
            .filter(|n| n.ends_with('A'))
            .cloned()
            .collect()
    }
}

#[derive(Debug, Default)]
struct MapBuilder {
    directions: Option<Vec<Direction>>,
    node_definitions: Vec<(NodeId, Node)>,
}

impl MapBuilder {
    fn add_line(&mut self, line: &str) -> Result<()> {
        if self.directions.is_none() {
            let directions = line
                .chars()
                .map(Direction::from_char)
                .collect::<Result<Vec<Direction>>>()?;
            self.directions = Some(directions);
            Ok(())
        } else if !line.is_empty() {
            let mut scanner = StringScanner::new(line);
            let node_id = NodeId::from_string_scanner(&mut scanner)?;
            scanner.expect_string(" = (")?;
            let left = NodeId::from_string_scanner(&mut scanner)?;
            scanner.expect_string(", ")?;
            let right = NodeId::from_string_scanner(&mut scanner)?;
            scanner.expect_string(")")?;
            self.node_definitions
                .push((node_id, Node::new(left, right)));
            Ok(())
        } else {
            Ok(())
        }
    }

    fn build(&self) -> Result<Map> {
        let nodes = self.node_definitions.iter().cloned().collect();
        let directions = if let Some(raw_directions) = &self.directions {
            raw_directions.to_vec()
        } else {
            panic!("urgh");
        };

        Ok(Map { directions, nodes })
    }
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
enum Direction {
    Right,
    Left,
}

impl Direction {
    fn from_char(c: char) -> Result<Self> {
        match c {
            'R' => Ok(Self::Right),
            'L' => Ok(Self::Left),
            _ => Err(CoreError::general(&format!("Bad direction char: {}", c))),
        }
    }
}

#[derive(Debug, Clone, Hash, Eq, PartialEq)]
struct NodeId(String);

impl NodeId {
    fn new(id: &str) -> Self {
        Self(id.to_string())
    }

    fn from_string_scanner(scanner: &mut StringScanner) -> Result<Self> {
        let mut id = String::new();
        for _ in 0..3 {
            if let Some(c) = scanner.peek() {
                id.push(c);
                scanner.advance();
            } else {
                return Err(CoreError::general(
                    "Reached end of string before end of node id",
                ));
            }
        }
        Ok(Self(id))
    }

    fn ends_with(&self, c: char) -> bool {
        self.0.ends_with(c)
    }
}

#[derive(Debug, Clone)]
struct Node {
    left: NodeId,
    right: NodeId,
}

impl Node {
    fn new(left: NodeId, right: NodeId) -> Self {
        Self { left, right }
    }

    fn next_node_id(&self, direction: Direction) -> NodeId {
        match direction {
            Direction::Left => self.left.clone(),
            Direction::Right => self.right.clone(),
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    fn simple_map() -> Map {
        let mut nodes = HashMap::new();

        nodes.insert(
            NodeId::new("AAA"),
            Node::new(NodeId::new("BBB"), NodeId::new("CCC")),
        );

        nodes.insert(
            NodeId::new("CCC"),
            Node::new(NodeId::new("ZZZ"), NodeId::new("GGG")),
        );

        Map {
            directions: vec![Direction::Right, Direction::Left],
            nodes,
        }
    }

    fn less_simple_map() -> Map {
        let mut nodes = HashMap::new();

        nodes.insert(
            NodeId::new("AAA"),
            Node::new(NodeId::new("BBB"), NodeId::new("BBB")),
        );

        nodes.insert(
            NodeId::new("BBB"),
            Node::new(NodeId::new("AAA"), NodeId::new("ZZZ")),
        );

        nodes.insert(
            NodeId::new("ZZZ"),
            Node::new(NodeId::new("ZZZ"), NodeId::new("ZZZ")),
        );

        Map {
            directions: vec![Direction::Left, Direction::Left, Direction::Right],
            nodes,
        }
    }

    #[test]
    fn can_follow_directions() {
        let map = simple_map();
        let num_steps = map.calculate_distance(NodeId::new("AAA"), &is_zzz);
        assert_eq!(num_steps, 2);
    }

    #[test]
    fn directions_are_cycled_until_destination() {
        let map = less_simple_map();
        let num_steps = map.calculate_distance(NodeId::new("AAA"), &is_zzz);
        assert_eq!(num_steps, 6);
    }

    #[test]
    fn test_parsing_and_solving() {
        let mut builder = MapBuilder::default();

        for line in [
            "RL",
            "",
            "AAA = (BBB, CCC)",
            "BBB = (DDD, EEE)",
            "CCC = (ZZZ, GGG)",
            "DDD = (DDD, DDD)",
            "EEE = (EEE, EEE)",
            "GGG = (GGG, GGG)",
            "ZZZ = (ZZZ, ZZZ)",
        ] {
            builder.add_line(line).unwrap();
        }

        let map = builder.build().unwrap();
        let num_steps = map.calculate_distance(NodeId::new("AAA"), &is_zzz);
        assert_eq!(num_steps, 2);
    }
}

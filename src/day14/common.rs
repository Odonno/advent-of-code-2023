use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Tile {
    RoundedRock,
    CubeRock,
    Empty,
}

pub type Position = (u8, u8);
pub type LavaMap = HashMap<Position, Tile>;

pub fn parse(input: &str) -> LavaMap {
    let mut lava_map = LavaMap::new();

    input.lines().enumerate().for_each(|(y, line)| {
        line.chars().enumerate().for_each(|(x, char)| {
            let tile = match char {
                '.' => Tile::Empty,
                '#' => Tile::CubeRock,
                'O' => Tile::RoundedRock,
                _ => panic!("Invalid char"),
            };

            lava_map.insert((x as u8, y as u8), tile);
        })
    });

    lava_map
}

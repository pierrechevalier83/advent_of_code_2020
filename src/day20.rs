use aoc_runner_derive::{aoc, aoc_generator};
use radix_fmt;
use std::collections::HashMap;
use std::convert::TryInto;
use std::fmt::{self, Debug, Formatter};

// find all outer edges
//
// rotation (4 each top -> right -> bottom -> left)
// flip
// 1100
// 0011
//
// for (int i = 0; i < x; i++)
//    num = (num ^ (1 << i));
// 1010011011010100
// 0010101101100101
//
//
//
// Representation: top, bottom, left, right (index, 4 u16 numbers)
//
// Mapping from each integer to 8 arrays of 4 u16s
//
// find any possibility with 2 consecutive edges (tor) that are unmatched or match already placed
// pieces
//

const TILE_SIZE: usize = 10;

type TileSlice = [bool; TILE_SIZE];

// Represent a tile's edge as a binary number
struct CompactEdge(u16);

impl From<&[bool]> for CompactEdge {
    fn from(slice: &[bool]) -> Self {
        let mut as_int = 0;
        for (index, bit) in slice.iter().rev().enumerate() {
            if *bit {
                as_int |= 1 << index;
            }
        }
        Self(as_int)
    }
}

impl Debug for CompactEdge {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}", radix_fmt::radix(self.0, 2))
    }
}

impl CompactEdge {
    fn flipped(&self) -> Self {
        Self(self.0.reverse_bits() >> 16 /*bits*/ - TILE_SIZE)
    }
}

struct CompactTile {
    edges: [CompactEdge; 4],
}

impl From<&Tile> for CompactTile {
    fn from(tile: &Tile) -> Self {
        CompactTile {
            edges: [tile.top(), tile.right(), tile.bottom(), tile.left()],
        }
    }
}

impl Debug for CompactTile {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        for edge in self.edges.iter() {
            writeln!(f, "{:?}", edge)?;
        }
        Ok(())
    }
}

struct Tile {
    data: [TileSlice; TILE_SIZE],
}

impl Tile {
    fn top(&self) -> CompactEdge {
        self.data[0].as_slice().into()
    }
    fn right(&self) -> CompactEdge {
        let right = self
            .data
            .iter()
            .map(|row| row[TILE_SIZE - 1])
            .collect::<Vec<_>>();
        right.as_slice().into()
    }
    fn bottom(&self) -> CompactEdge {
        CompactEdge::from(self.data[TILE_SIZE - 1].as_slice()).flipped()
    }
    fn left(&self) -> CompactEdge {
        let left = self.data.iter().map(|row| row[0]).collect::<Vec<_>>();
        CompactEdge::from(left.as_slice()).flipped()
    }
}

impl From<&str> for Tile {
    fn from(s: &str) -> Self {
        let vec = s
            .split_terminator('\n')
            .map(|row| {
                let bits = row.chars().map(|c| c == '#').collect::<Vec<_>>();
                bits.try_into().unwrap()
            })
            .collect::<Vec<TileSlice>>();
        Self {
            data: vec.try_into().unwrap(),
        }
    }
}

impl Debug for Tile {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        for row in self.data.iter() {
            for cell in row {
                // Colour to match the desert theme. Some might say: essential
                let c = if *cell { '🟫' } else { '🟧' };
                write!(f, "{}", c)?;
            }
            writeln!(f, "")?;
        }
        Ok(())
    }
}

struct Jigsaw {
    tiles: HashMap<usize, Tile>,
    compact: HashMap<usize, CompactTile>,
}

impl From<&str> for Jigsaw {
    fn from(s: &str) -> Self {
        let tiles = s
            .split_terminator("\n\n")
            .map(|tile_region| {
                let (id_line, tile_lines) = tile_region.split_once('\n').unwrap();
                let id: usize = id_line
                    .replace("Tile ", "")
                    .replace(":", "")
                    .parse()
                    .unwrap();
                let tile = tile_lines.into();
                (id, tile)
            })
            .collect::<HashMap<_, _>>();
        let compact = tiles
            .iter()
            .map(|(id, tile)| (*id, CompactTile::from(tile)))
            .collect();
        Self { tiles, compact }
    }
}

impl Debug for Jigsaw {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        for (id, tile) in self.tiles.iter() {
            writeln!(f, "⪘⪘⪘⪘⪘⪘⪘⪘⪘⪘⪘⪘⪘⪘⪘⪘⪘⪘⪘")?;
            writeln!(f, "\nTile {}:\n", id)?;
            writeln!(f, "{:?}", tile)?;
            writeln!(f, "{:?}", self.compact[id])?;
            writeln!(f, "⪗⪗⪗⪗⪗⪗⪗⪗⪗⪗⪗⪗⪗⪗⪗⪗⪗⪗⪗")?;
        }
        Ok(())
    }
}

#[aoc_generator(day20)]
fn parse_input(s: &str) -> Jigsaw {
    s.into()
}

#[aoc(day20, part1)]
fn part1(jig: &Jigsaw) -> usize {
    println!("{:#?}", jig);
    42
}

#[cfg(test)]
mod tests {
    use super::*;
    fn input() -> Jigsaw {
        parse_input(include_str!("../input/2020/day20.txt"))
    }
    #[test]
    fn test_part1() {
        assert_eq!(part1(&input()), 42)
    }
}

use aoc_runner_derive::{aoc, aoc_generator};
use radix_fmt;
use std::collections::HashMap;
use std::convert::TryInto;
use std::fmt::{self, Debug, Formatter};
use std::iter::once;

// mapping edge -> candidates, permutation
//
// find ones with 2 unmatched edges
// find one with one unmatched edge
//
//
// mapping of id to edge
//
// find all outer edges
// orient them
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
type TileId = usize;
type PermutationId = usize;

// Represent a tile's edge as a binary number
#[derive(Clone, Copy, Eq, PartialEq, Hash)]
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

const TOP: usize = 0;
const RIGHT: usize = 1;
const BOTTOM: usize = 2;
const LEFT: usize = 3;

#[derive(Clone)]
struct CompactTile {
    // All four edges ordered as such: top, right, bottom, left
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

impl CompactTile {
    // Same tile in any 3 flip states for each of the 4 orientations
    fn all_permutations(&self) -> impl Iterator<Item = Self> + '_ {
        self.all_orientations()
            .flat_map(|x| x.all_flips().collect::<Vec<_>>())
    }
    fn all_orientations(&self) -> impl Iterator<Item = Self> + '_ {
        (0..4).map(move |index| {
            let mut edges = self.edges.clone();
            edges.rotate_right(index);
            Self { edges }
        })
    }
    fn all_flips(&self) -> impl Iterator<Item = Self> + '_ {
        once(self.clone())
            .chain(once(self.x_flip()))
            .chain(once(self.y_flip()))
    }
    fn x_flip(&self) -> Self {
        let mut edges = self.edges.clone();
        edges.swap(TOP, BOTTOM);
        edges[RIGHT] = edges[RIGHT].flipped();
        edges[LEFT] = edges[LEFT].flipped();
        Self { edges }
    }
    fn y_flip(&self) -> Self {
        let mut edges = self.edges.clone();
        edges.swap(RIGHT, LEFT);
        edges[TOP] = edges[TOP].flipped();
        edges[BOTTOM] = edges[BOTTOM].flipped();
        Self { edges }
    }
    fn left(&self) -> CompactEdge {
        self.edges[LEFT]
    }
    fn top(&self) -> CompactEdge {
        self.edges[TOP]
    }
    fn bottom(&self) -> CompactEdge {
        self.edges[BOTTOM]
    }
    fn right(&self) -> CompactEdge {
        self.edges[RIGHT]
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
    tiles: HashMap<TileId, Tile>,
    compact: HashMap<TileId, CompactTile>,
    edge_mapping: HashMap<CompactEdge, Vec<TileId>>,
}

impl From<&str> for Jigsaw {
    fn from(s: &str) -> Self {
        let tiles = s
            .split_terminator("\n\n")
            .map(|tile_region| {
                let (id_line, tile_lines) = tile_region.split_once('\n').unwrap();
                let id: TileId = id_line
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
        let mut me = Self {
            tiles,
            compact,
            edge_mapping: HashMap::new(),
        };
        me.edge_mapping = me.edge_mapping();
        me
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

impl Jigsaw {
    // get a mapping from an edge to the tiles that contain it (raw or flipped)
    // permutations will only come in later
    fn edge_mapping(&self) -> HashMap<CompactEdge, Vec<TileId>> {
        let mut mapping = HashMap::new();
        for (id, tile) in self.compact.iter() {
            for edge in tile.edges.iter() {
                let that_edge = mapping.entry(*edge).or_insert(vec![]);
                that_edge.push(*id);
                let flipped_edge = mapping.entry(edge.flipped()).or_insert(vec![]);
                flipped_edge.push(*id);
            }
        }
        mapping
    }
    fn num_unmatched_edges(&self, tile: &CompactTile) -> usize {
        tile.edges
            .iter()
            .filter(|edge| self.edge_mapping[edge].len() == 1)
            .count()
    }

    fn is_corner(&self, tile: &CompactTile) -> bool {
        self.num_unmatched_edges(tile) == 2
    }
    // Find the permutation of the tile "id", which sits to the right of "left" and to the
    // bottom of "top"
    fn find_permutation(
        &self,
        id: TileId,
        left: Option<TileId>,
        top: Option<TileId>,
    ) -> PermutationId {
        let tile = self.compact[&id].clone();
        let permutation = tile
            .all_permutations()
            .enumerate()
            .find_map(move |(perm_id, perm)| {
                let mut top_neighbours =
                    self.edge_mapping[&perm.top()].iter().filter(|t| **t != id);
                let top_matches = if top.is_none() {
                    top_neighbours.next().is_none()
                } else {
                    top_neighbours.next().copied() == top && top_neighbours.next().is_none()
                };
                let mut left_neighbours =
                    self.edge_mapping[&perm.left()].iter().filter(|t| **t != id);
                let left_matches = if left.is_none() {
                    left_neighbours.next().is_none()
                } else {
                    left_neighbours.next().copied() == left && left_neighbours.next().is_none()
                };
                if top_matches && left_matches {
                    Some(perm_id)
                } else {
                    None
                }
            })
            .expect("Expected to be able to permut the corner into a top left position");
        permutation
    }
    fn tile_with_shared_edge(&self, edge: CompactEdge, tile_id: TileId) -> Option<TileId> {
        self.edge_mapping[&edge]
            .iter()
            .filter(|id| **id != tile_id)
            .next()
            .copied()
    }
    fn get_tile(&self, id: TileId, perm: PermutationId) -> Option<CompactTile> {
        self.compact[&id].all_permutations().nth(perm)
    }
    fn find_tile(
        &self,
        left: Option<(TileId, PermutationId)>,
        top: Option<(TileId, PermutationId)>,
    ) -> (TileId, PermutationId) {
        let id = if left.is_none() && top.is_none() {
            self.compact
                .iter()
                .find(|(_id, tile)| self.is_corner(tile))
                .map(|(id, _tile)| *id)
                .expect("Expected to find a corner tile")
        } else if top.is_none() {
            let (left_id, perm) = left.unwrap();
            let left_tile = self.get_tile(left_id, perm).unwrap();
            self.tile_with_shared_edge(left_tile.right(), left_id)
                .unwrap()
        } else {
            let (top_id, perm) = top.unwrap();
            let top_tile = self.get_tile(top_id, perm).unwrap();
            self.tile_with_shared_edge(top_tile.bottom(), top_id)
                .unwrap()
        };
        (
            id,
            self.find_permutation(id, left.map(|(id, _)| id), top.map(|(id, _)| id)),
        )
    }
    // It appears all edges are unique, which is allows us to be naive
    fn assemble_jigsaw(&self) -> Vec<Vec<(TileId, PermutationId)>> {
        let mut assembled = Vec::<Vec<(TileId, PermutationId)>>::new();
        let mut current_row = vec![];
        let mut col_index = 0;
        loop {
            let top = assembled.last().map(|last_row| last_row[col_index]);
            let left = current_row.last().copied();
            let (id, perm) = self.find_tile(left, top);
            let right = self.get_tile(id, perm).map(|x| x.right()).unwrap();
            let bottom = self.get_tile(id, perm).map(|x| x.bottom()).unwrap();
            current_row.push((id, perm));
            if self.tile_with_shared_edge(right, id).is_some() {
                col_index += 1;
            } else if self.tile_with_shared_edge(bottom, id).is_some() {
                assembled.push(current_row.clone());
                current_row = vec![];
                col_index = 0;
            } else {
                assembled.push(current_row.clone());
                return assembled;
            }
        }
    }
}

#[aoc_generator(day20)]
fn parse_input(s: &str) -> Jigsaw {
    s.into()
}

#[aoc(day20, part1)]
fn part1(jig: &Jigsaw) -> usize {
    let assembled = jig.assemble_jigsaw();
    let first_row = assembled[0].clone();
    let last_row = assembled.last().unwrap();
    first_row[0].0 * first_row.last().unwrap().0 * last_row[0].0 * last_row.last().unwrap().0
}

#[cfg(test)]
mod tests {
    use super::*;
    fn input() -> Jigsaw {
        parse_input(include_str!("../input/2020/day20.txt"))
    }
    const EXAMPLE: &'static str = "Tile 2311:
..##.#..#.
##..#.....
#...##..#.
####.#...#
##.##.###.
##...#.###
.#.#.#..##
..#....#..
###...#.#.
..###..###

Tile 1951:
#.##...##.
#.####...#
.....#..##
#...######
.##.#....#
.###.#####
###.##.##.
.###....#.
..#.#..#.#
#...##.#..

Tile 1171:
####...##.
#..##.#..#
##.#..#.#.
.###.####.
..###.####
.##....##.
.#...####.
#.##.####.
####..#...
.....##...

Tile 1427:
###.##.#..
.#..#.##..
.#.##.#..#
#.#.#.##.#
....#...##
...##..##.
...#.#####
.#.####.#.
..#..###.#
..##.#..#.

Tile 1489:
##.#.#....
..##...#..
.##..##...
..#...#...
#####...#.
#..#.#.#.#
...#.#.#..
##.#...##.
..##.##.##
###.##.#..

Tile 2473:
#....####.
#..#.##...
#.##..#...
######.#.#
.#...#.#.#
.#########
.###.#..#.
########.#
##...##.#.
..###.#.#.

Tile 2971:
..#.#....#
#...###...
#.#.###...
##.##..#..
.#####..##
.#..####.#
#..#.#..#.
..####.###
..#.#.###.
...#.#.#.#

Tile 2729:
...#.#.#.#
####.#....
..#.#.....
....#..#.#
.##..##.#.
.#.####...
####.#.#..
##.####...
##..#.##..
#.##...##.

Tile 3079:
#.#.#####.
.#..######
..#.......
######....
####.#..#.
.#...#.##.
#.#####.##
..#.###...
..#.......
..#.###...";
    #[test]
    fn test_example() {
        assert_eq!(20899048083289, part1(&parse_input(EXAMPLE)))
    }
    #[test]
    fn test_part1() {
        assert_eq!(0, part1(&input()))
    }
}

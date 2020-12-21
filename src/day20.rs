use aoc_runner_derive::{aoc, aoc_generator};
use radix_fmt;
use rustc_hash::FxHashMap;
use std::convert::TryInto;
use std::fmt::{self, Debug, Formatter};
use std::iter::{once, repeat};

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
        once(self.clone()).chain(once(self.x_flip()))
    }
    fn x_flip(&self) -> Self {
        let mut edges = self.edges.clone();
        edges.swap(TOP, BOTTOM);
        edges[RIGHT] = edges[RIGHT].flipped();
        edges[LEFT] = edges[LEFT].flipped();
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

#[derive(Clone)]
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
    // clockwise rotation
    fn rotate(&self) -> Self {
        let mut next = self.data.clone();
        for ii in 0..TILE_SIZE {
            for jj in 0..TILE_SIZE {
                next[ii][jj] = self.data[TILE_SIZE - jj - 1][ii];
            }
        }
        Self { data: next }
    }
    fn flip_x(&self) -> Self {
        let mut next = self.data.clone();
        for ii in 0..TILE_SIZE {
            next[ii] = self.data[TILE_SIZE - ii - 1];
        }
        Self { data: next }
    }
    fn flip(&self, num_flips: usize) -> Self {
        match num_flips {
            0 => self.clone(),
            1 => self.flip_x(),
            _ => panic!("Expected one of 3 valid values for num_flips"),
        }
    }
    fn with_permutation(&self, perm: PermutationId) -> Self {
        // See CompactTile::apply_permutation for the source of truth on the order of permutations
        let mut permuted = self.clone();
        let num_rotations = perm / 2;
        let num_flips = perm % 2;
        for _ in 0..num_rotations {
            permuted = permuted.rotate();
        }
        permuted = permuted.flip(num_flips);
        permuted
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
    tile_ids: Vec<TileId>,
    tiles: Vec<Tile>,
    compact: Vec<CompactTile>,
    edge_mapping: FxHashMap<CompactEdge, Vec<usize>>,
}

impl From<&str> for Jigsaw {
    fn from(s: &str) -> Self {
        let (tile_ids, tiles): (Vec<_>, Vec<Tile>) = s
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
            .unzip();
        let compact = tiles.iter().map(|tile| CompactTile::from(tile)).collect();
        let mut me = Self {
            tile_ids,
            tiles,
            compact,
            edge_mapping: FxHashMap::default(),
        };
        me.edge_mapping = me.edge_mapping();
        me
    }
}

impl Debug for Jigsaw {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        for ((id, tile), compact) in self
            .tile_ids
            .iter()
            .zip(self.tiles.iter())
            .zip(self.compact.iter())
        {
            writeln!(f, "⪘⪘⪘⪘⪘⪘⪘⪘⪘⪘⪘⪘⪘⪘⪘⪘⪘⪘⪘")?;
            writeln!(f, "\nTile {}:\n", id)?;
            writeln!(f, "{:?}", tile)?;
            writeln!(f, "{:?}", compact)?;
            writeln!(f, "⪗⪗⪗⪗⪗⪗⪗⪗⪗⪗⪗⪗⪗⪗⪗⪗⪗⪗⪗")?;
        }
        Ok(())
    }
}

impl Jigsaw {
    // get a mapping from an edge to the tiles that contain it (raw or flipped)
    // permutations will only come in later
    fn edge_mapping(&self) -> FxHashMap<CompactEdge, Vec<TileId>> {
        let mut mapping = FxHashMap::default();
        for (index, tile) in self.compact.iter().enumerate() {
            for edge in tile.edges.iter() {
                let that_edge = mapping.entry(*edge).or_insert(vec![]);
                that_edge.push(index);
                let flipped_edge = mapping.entry(edge.flipped()).or_insert(vec![]);
                flipped_edge.push(index);
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
        index: usize,
        left: Option<usize>,
        top: Option<usize>,
    ) -> PermutationId {
        let tile = self.compact[index].clone();
        let permutation = tile
            .all_permutations()
            .enumerate()
            .find_map(move |(perm_id, perm)| {
                let mut top_neighbours = self.edge_mapping[&perm.top()]
                    .iter()
                    .filter(|t| **t != index);
                let top_matches = if top.is_none() {
                    top_neighbours.next().is_none()
                } else {
                    top_neighbours.next().copied() == top && top_neighbours.next().is_none()
                };
                let mut left_neighbours = self.edge_mapping[&perm.left()]
                    .iter()
                    .filter(|t| **t != index);
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
    fn get_tile(&self, index: usize, perm: PermutationId) -> Option<CompactTile> {
        self.compact[index].all_permutations().nth(perm)
    }
    fn find_tile(
        &self,
        left: Option<(usize, PermutationId)>,
        top: Option<(usize, PermutationId)>,
    ) -> (TileId, PermutationId) {
        let index = if left.is_none() && top.is_none() {
            self.compact
                .iter()
                .position(|tile| self.is_corner(tile))
                .expect("Expected to find a corner tile")
        } else if top.is_none() {
            let (left_index, perm) = left.unwrap();
            let left_tile = self.get_tile(left_index, perm).unwrap();
            self.tile_with_shared_edge(left_tile.right(), left_index)
                .unwrap()
        } else {
            let (top_index, perm) = top.unwrap();
            let top_tile = self.get_tile(top_index, perm).unwrap();
            self.tile_with_shared_edge(top_tile.bottom(), top_index)
                .unwrap()
        };
        (
            index,
            self.find_permutation(
                index,
                left.map(|(index, _)| index),
                top.map(|(index, _)| index),
            ),
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
            let (index, perm) = self.find_tile(left, top);
            let right = self.get_tile(index, perm).map(|x| x.right()).unwrap();
            let bottom = self.get_tile(index, perm).map(|x| x.bottom()).unwrap();
            current_row.push((index, perm));
            if self.tile_with_shared_edge(right, index).is_some() {
                col_index += 1;
            } else if self.tile_with_shared_edge(bottom, index).is_some() {
                assembled.push(current_row.clone());
                current_row = vec![];
                col_index = 0;
            } else {
                assembled.push(current_row.clone());
                return assembled;
            }
        }
    }
    fn picture(&self) -> Vec<Vec<bool>> {
        let mut output = Vec::<Vec<bool>>::new();
        for (row_index, row) in self.assemble_jigsaw().iter().enumerate() {
            for (col_index, (index, perm)) in row.iter().enumerate() {
                let tile = self.tiles[*index].with_permutation(*perm);
                for (tile_row_index, tile_row) in tile.data.iter().enumerate() {
                    if tile_row_index != 0 && tile_row_index != TILE_SIZE - 1 {
                        if col_index == 0 {
                            output.push(vec![]);
                        }
                        output[row_index * (TILE_SIZE - 2) + (tile_row_index - 1)]
                            .extend_from_slice(&tile_row[1..(TILE_SIZE - 1)]);
                    }
                }
            }
        }
        output
    }
}

fn sea_monsters(sea: &[Vec<bool>]) -> Vec<Vec<bool>> {
    let monster = [
        "                  # ",
        "#    ##    ##    ###",
        " #  #  #  #  #  #   ",
    ];
    let n_rows = monster.len();
    let n_cols = monster[0].chars().count();
    let sea_monster = monster
        .iter()
        .enumerate()
        .flat_map(|(row_index, row)| {
            row.chars().enumerate().filter_map(move |(col_index, c)| {
                if c == '#' {
                    Some((row_index, col_index))
                } else {
                    None
                }
            })
        })
        .collect::<Vec<_>>();

    let mut output = repeat(repeat(false).take(sea[0].len()).collect::<Vec<_>>())
        .take(sea.len())
        .collect::<Vec<_>>();
    for row in 0..(sea.len() - n_rows) {
        for col in 0..(sea[0].len() - n_cols) {
            if sea_monster.iter().all(|(r, c)| sea[row + r][col + c]) {
                sea_monster
                    .iter()
                    .for_each(|(r, c)| output[row + r][col + c] = true);
            }
        }
    }
    output
}

fn rotate(data: &[Vec<bool>]) -> Vec<Vec<bool>> {
    let mut next = data.iter().cloned().collect::<Vec<_>>();
    for ii in 0..data.len() {
        for jj in 0..data[0].len() {
            next[ii][jj] = data[data[0].len() - jj - 1][ii];
        }
    }
    next
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
    [
        first_row.first(),
        first_row.last(),
        last_row.first(),
        last_row.last(),
    ]
    .iter()
    .map(|maybe| jig.tile_ids[maybe.unwrap().0])
    .product()
}

#[aoc(day20, part2)]
fn part2(jig: &Jigsaw) -> usize {
    let mut picture = jig.picture();
    let num_cells = picture.iter().flatten().filter(|cell| **cell).count();
    let mut num_monster_cells = 0;
    let mut n_rotations = 0;
    while num_monster_cells == 0 && n_rotations < 4 {
        n_rotations += 1;
        let sea_monsters = sea_monsters(&picture);
        num_monster_cells = sea_monsters.iter().flatten().filter(|cell| **cell).count();
        if num_monster_cells == 0 {
            // Note: found them with only rotation and no flip, so didn't bother flipping
            picture = rotate(&picture);
        }
    }
    num_cells - num_monster_cells
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
        assert_eq!(140656720229539, part1(&input()))
    }
    #[test]
    fn test_part2() {
        assert_eq!(1885, part2(&input()))
    }
}

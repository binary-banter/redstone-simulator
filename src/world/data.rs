use crate::blocks::facing::Facing;
use crate::blocks::CBlock;
use crate::world::schematic::{SchemBlockEntity, SchemFormat};
use itertools::{iproduct, Itertools};
use std::collections::HashMap;
use std::ops::{Index, IndexMut};

pub struct WorldData(pub Vec<Vec<Vec<Vec<CBlock>>>>);

pub type TileMap<'a> = HashMap<(usize, usize, usize), &'a SchemBlockEntity>;
type Palette = Vec<Vec<CBlock>>;

impl WorldData {
    /// Returns palette made from `SchemFormat`.
    fn create_palette(format: &SchemFormat) -> Palette {
        format
            .palette
            .iter()
            .sorted_by_key(|(_, i)| *i)
            .map(|(id, _)| CBlock::from_id(id.as_str()))
            .collect()
    }

    /// Creates instance of `WorldData` from `SchemFormat` and `TileMap`.
    pub fn from_format(format: &SchemFormat, tile_map: &TileMap) -> WorldData {
        let palette = Self::create_palette(format);

        let height = format.height as usize;
        let length = format.length as usize;
        let width = format.width as usize;

        let mut world = vec![vec![vec![vec![]; length]; height]; width];

        // Closure that returns next palette index in `block_data`.
        let mut read_head = 0;
        let mut read_next = || {
            let mut ix: usize = 0;
            for j in 0.. {
                let next = format.block_data[read_head];
                ix |= (next as usize & 0b0111_1111) << (j * 7);
                read_head += 1;

                if next >= 0 {
                    break;
                }
            }
            ix
        };

        // construct blocks from palette and use entity data to update them.
        for (y, z, x) in iproduct!(0..height, 0..length, 0..width) {
            world[x][y][z] = palette[read_next()]
                .iter()
                .cloned()
                .map(|mut b| {
                    match &mut b {
                        CBlock::Comparator(v) => v.update_from_tile((x, y, z), tile_map),
                        CBlock::Probe(v) => v.update_from_tile((x, y, z), tile_map),
                        _ => {}
                    }
                    b
                })
                .collect();
        }

        WorldData(world)
    }
}

/// Returns the coordinates of neighbouring blocks relative to the given position.
pub fn neighbours((x, y, z): (usize, usize, usize)) -> impl Iterator<Item = (usize, usize, usize)> {
    [
        (x.wrapping_sub(1), y, z),
        (x.wrapping_add(1), y, z),
        (x, y.wrapping_sub(1), z),
        (x, y.wrapping_add(1), z),
        (x, y, z.wrapping_sub(1)),
        (x, y, z.wrapping_add(1)),
    ]
    .into_iter()
}

/// Returns the coordinates and facing of neighbouring blocks relative to the given position.
pub fn neighbours_and_facings(
    (x, y, z): (usize, usize, usize),
) -> impl Iterator<Item = ((usize, usize, usize), Facing)> {
    [
        ((x.wrapping_sub(1), y, z), Facing::West),
        ((x.wrapping_add(1), y, z), Facing::East),
        ((x, y.wrapping_sub(1), z), Facing::Down),
        ((x, y.wrapping_add(1), z), Facing::Up),
        ((x, y, z.wrapping_sub(1)), Facing::North),
        ((x, y, z.wrapping_add(1)), Facing::South),
    ]
    .into_iter()
}

const EMPTY_VEC: &Vec<CBlock> = &Vec::new();

impl Index<(usize, usize, usize)> for WorldData {
    type Output = Vec<CBlock>;

    fn index(&self, (x, y, z): (usize, usize, usize)) -> &Self::Output {
        self.0
            .get(x)
            .and_then(|l| l.get(y).and_then(|l| l.get(z)))
            .unwrap_or(EMPTY_VEC)
    }
}

impl IndexMut<(usize, usize, usize)> for WorldData {
    fn index_mut(&mut self, (x, y, z): (usize, usize, usize)) -> &mut Self::Output {
        &mut self.0[x][y][z]
    }
}

use crate::{InvertDelta, Space};
use std::ops::{Index, IndexMut};

/*
Uses a y-up coordinate system

Cube as viewed from above
(0,0,length) - - - - (width,0,length)
            |       |
            |       |
     (0,0,0) - - - - (width,0,0)
*/

#[derive(Debug)]
pub struct CubeGrid<T> {
    cells: Box<[T]>,
    width: isize,
    length: isize,
    height: isize,
}

impl InvertDelta for (isize, isize, isize) {
    fn invert_delta(&self) -> Self {
        let (dx, dy, dz) = *self;
        (-dx, -dy, -dz)
    }
}

impl<T> CubeGrid<T> {
    // width - x axis
    // length - z axis
    // height - y axis
    // init_fn - callback to set the initial state of each cell based on coordinate
    pub fn new(
        width: isize,
        length: isize,
        height: isize,
        init_fn: impl Fn(isize, isize, isize) -> T,
    ) -> Self {
        let mut cells = Vec::new();
        for y in 0..height {
            for z in 0..length {
                for x in 0..width {
                    cells.push(init_fn(x, y, z));
                }
            }
        }
        Self {
            cells: cells.into_boxed_slice(),
            width,
            length,
            height,
        }
    }
}

// Access to a certain cells possible states
impl<T: 'static> Index<<CubeGrid<T> as Space<T>>::Coordinate> for CubeGrid<T> {
    type Output = T;

    fn index(&self, index: <CubeGrid<T> as Space<T>>::Coordinate) -> &Self::Output {
        let (x, y, z) = index;

        // Return the cell corresponding to the coordinate - depends on order cells are initialized
        // in new()
        // (1,0,0) - is at [1]
        // (0,0,1) - is at [self.width]
        // (0,1,0) - is at [self.width * self.length]
        &self.cells[((y * self.width * self.length) + (z * self.width) + x) as usize]
    }
}

// Mutable access to a certain cells possible states
impl<T: 'static> IndexMut<<CubeGrid<T> as Space<T>>::Coordinate> for CubeGrid<T> {
    fn index_mut(&mut self, index: <CubeGrid<T> as Space<T>>::Coordinate) -> &mut Self::Output {
        let (x, y, z) = index;
        &mut self.cells[((y * self.width * self.length) + (z * self.width) + x) as usize]
    }
}

impl<T: 'static> Space<T> for CubeGrid<T> {
    type Coordinate = (isize, isize, isize);
    type CoordinateDelta = (isize, isize, isize);

    fn coordinate_list(&self) -> Box<[Self::Coordinate]> {
        let mut coords = Vec::new();

        for y in 0..self.height {
            for z in 0..self.length {
                for x in 0..self.width {
                    coords.push((x, y, z));
                }
            }
        }
        coords.into_boxed_slice()
    }

    fn neighbors(
        &self,
        coord: Self::Coordinate,
        neighbor_directions: &[Self::CoordinateDelta],
        neighbors: &mut [Option<Self::Coordinate>],
    ) {
        assert!(neighbor_directions.len() <= neighbors.len());

        let (x, y, z) = coord;
        for i in 0..neighbor_directions.len() {
            let (dx, dy, dz) = neighbor_directions[i];
            let (nx, ny, nz) = (x + dx, y + dy, z + dz);
            if nx.clamp(0, self.width - 1) == nx
                && nz.clamp(0, self.length - 1) == nz
                && ny.clamp(0, self.height - 1) == ny
            {
                neighbors[i] = Some((nx, ny, nz));
            } else {
                neighbors[i] = None;
            }
        }
    }
}

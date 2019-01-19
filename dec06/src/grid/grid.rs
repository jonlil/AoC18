use std::cmp;

#[derive(Debug)]
pub struct Grid2D<T> {
    width: usize,
    height: usize,
    grid: Vec<T>,
}

/// Immutable iterator over a Grid2D
#[derive(Debug)]
pub struct GridIter<'a, T: 'a> {
    slice: &'a [T],
    x: i32,
    y: i32,
    /// distance between end of one span and beginning of another
    stride: usize,
    x1: i32,
    x2: i32,
    y2: i32,
}

impl<T: Clone> Grid2D<T> {
    pub fn new() -> Grid2D<T> {
        Grid2D {
            width: 0,
            height: 0,
            grid: Vec::new(),
        }
    }

    pub fn new_sized(width: usize, height: usize, val: &T) -> Grid2D<T> {
        Grid2D {
            width,
            height,
            grid: vec![val.clone(); width * height],
        }
    }

    /// Coordinates to offset in array
    fn to_offset(&self, x: i32, y: i32) -> Option<usize> {
        // This makes use of the fact that negative values casted to usize
        // will be larger than the width/height.
        if (x as usize) < self.width && (y as usize) < self.height {
            Some((y as usize) * self.width + (x as usize))
        } else {
            None
        }
    }

    /// 2D map lookup
    pub fn get(&self, x: i32, y: i32) -> Option<&T> {
        self.to_offset(x, y).map(move |ofs| &self.grid[ofs])
    }

    /// 2D map put
    pub fn set(&mut self, x: i32, y: i32, tile_in: T) -> bool {
        if let Some(ofs) = self.to_offset(x, y) {
            self.grid[ofs] = tile_in;
            true
        } else {
            false
        }
    }

    /// Clip a rectangle against grid boundaries
    #[allow(dead_code)]
    fn clip_rect(&self, x1: i32, y1: i32, x2: i32, y2: i32) -> (i32, i32, i32, i32) {
        let x1 = cmp::min(cmp::max(x1, 0), self.width as i32);
        let x2 = cmp::min(cmp::max(x2, 0), self.width as i32);
        let y1 = cmp::min(cmp::max(y1, 0), self.height as i32);
        let y2 = cmp::min(cmp::max(y2, 0), self.height as i32);
        if x2 <= x1 || y2 <= y1 {
            // left with invalid rectangle, normalize it to 0,0
            (0, 0, 0, 0)
        } else {
            (x1, y1, x2, y2)
        }
    }

    /// Iterate over a rectangle
    #[allow(dead_code)]
    pub fn iter_region(&self, x1: i32, y1: i32, x2: i32, y2: i32) -> GridIter<T> {
        let (x1, y1, x2, y2) = self.clip_rect(x1, y1, x2, y2);
        let ofs = self.to_offset(x1, y1).unwrap_or(0);
        let stride = self.width - (x2 - x1) as usize;
        GridIter {
            slice: self.grid.split_at(ofs).1,
            x: x1,
            y: y1,
            stride,
            x1,
            x2,
            y2,
        }
    }

    /// Iterate over entire grid
    pub fn iter(&mut self) -> GridIter<T> {
        self.iter_region(0, 0, self.width as i32, self.height as i32)
    }
}

impl<'a, T> Iterator for GridIter<'a, T> {
    type Item = ((i32, i32), &'a T);

    fn next(&mut self) -> Option<Self::Item> {
        if self.y < self.y2 {
            let (cell, rest) = self.slice.split_at(1);
            let ret = Some(((self.x, self.y), &cell[0]));
            self.slice = rest;

            // Advance
            self.x += 1;
            if self.x >= self.x2 {
                self.x = self.x1;
                self.y += 1;
                if self.y < self.y2 { // don't advance out of slice
                    self.slice = self.slice.split_at(self.stride).1;
                }
            }

            ret
        } else {
            // out of range
            None
        }
    }
}

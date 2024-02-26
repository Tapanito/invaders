use crate::{EMPTY_CELL, NUM_COLS, NUM_ROWS};
use std::{
    error::Error,
    ops::{Index, IndexMut},
};

type Row = Vec<&'static str>;

pub struct Frame(Vec<Row>);

impl Frame {
    pub fn new() -> Self {
        let mut cols = Vec::with_capacity(NUM_COLS);
        for _ in 0..cols.capacity() {
            let mut col = Row::with_capacity(NUM_ROWS);
            for _ in 0..col.capacity() {
                col.push(EMPTY_CELL);
            }
            cols.push(col);
        }

        Frame(cols)
    }

    pub fn size(&self) -> usize {
        self.0.len() * self.0[0].len()
    }

    pub fn num_cols(&self) -> usize {
        self.0.len()
    }

    pub fn num_rows(&self) -> usize {
        self.0[0].len()
    }

    pub fn draw<T: Drawable>(&mut self, drawable: &T) {
        drawable.values().iter().for_each(|f| self[f.0][f.1] = f.2);
    }

    pub fn all_in_frame(&self, positions: Vec<(i32, i32)>) -> bool {
        positions
            .iter()
            .all(|f| (f.0 as usize) < self.num_cols() && (f.1 as usize) < self.num_rows())
    }
}

impl<'a> IntoIterator for &'a Frame {
    type Item = (usize, usize, &'static str);
    type IntoIter = FrameIterator<'a>;

    fn into_iter(self) -> Self::IntoIter {
        FrameIterator {
            frame: self,
            col_index: 0,
            row_index: 0,
        }
    }
}

impl Index<usize> for Frame {
    type Output = Row;

    fn index(&self, index: usize) -> &Self::Output {
        Index::index(&self.0, index)
    }
}

impl IndexMut<usize> for Frame {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        IndexMut::index_mut(&mut self.0, index)
    }
}
pub struct FrameIterator<'a> {
    frame: &'a Frame,
    col_index: usize,
    row_index: usize,
}

impl<'a> Iterator for FrameIterator<'a> {
    type Item = (usize, usize, &'static str);

    fn next(&mut self) -> Option<Self::Item> {
        if self.col_index >= self.frame.num_cols() {
            return None;
        }

        let value = (
            self.col_index,
            self.row_index,
            self.frame.0[self.col_index][self.row_index],
        );
        if self.row_index + 1 >= self.frame.num_rows() {
            self.row_index = 0;
            self.col_index += 1;
        } else {
            self.row_index += 1;
        }

        Some(value)
    }
}

pub trait Drawable {
    fn values(&self) -> Vec<(usize, usize, &'static str)>;
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn frame_new() {
        let frame = Frame::new();
        assert_eq!(frame.0.len(), NUM_COLS);
        for col in frame.0 {
            assert_eq!(col.len(), NUM_ROWS);
            for row in col {
                assert_eq!(row, EMPTY_CELL)
            }
        }
    }

    #[test]
    fn test_iterator() {
        let frame = Frame::new();
        let expected_cells = frame.size();
        let mut actual_cells = 0;
        for _ in &frame {
            actual_cells += 1;
        }

        assert_eq!(expected_cells, actual_cells)
    }
}

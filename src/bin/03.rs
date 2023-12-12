use std::{iter::Enumerate, slice::Iter};

use itertools::Itertools;

advent_of_code::solution!(3);

/// An iterator over a 2D matrix that returns the current item, its position within the matrix, its position within
/// the window and the window itself.
///
/// The window is a `Vec` of `Vec`s, where each `Vec` is a row of the window.
///
/// **IMPORTANT**: the window is not guaranteed to be square, so the number of columns and row may differ between windows.
/// This is because the window is always centered on the current item, so if the current item is on the edge of the matrix,
/// the window will be smaller than if the current item is in the middle of the matrix.
/// Each Vec in a window is guaranteed to have the same length as the other Vecs in the window.
///
/// Unequal number of columns between each line is undefined behavior.
///
///
#[derive(Debug)]
struct WindowIterator<'a, T: 'a> {
    size: (usize, usize),
    data: &'a Vec<Vec<T>>,

    lines: Enumerate<Iter<'a, Vec<T>>>,
    current_line: Option<(usize, Enumerate<Iter<'a, T>>)>,
}

impl<'a, T> WindowIterator<'a, T> {
    pub fn new(data: &'a Vec<Vec<T>>, size: (usize, usize)) -> Self {
        let mut lines = data.iter().enumerate();
        let current_line = lines
            .next()
            .map(|(ln, columns)| (ln, columns.iter().enumerate()));

        let (width, height) = size;
        assert_eq!(width % 2, 1, "width must be odd");
        assert_eq!(height % 2, 1, "height must be odd");

        let half_width = (width - 1) / 2;
        let half_height = (height - 1) / 2;

        assert!((lines.len() - 1 + half_height) <= usize::MAX);

        Self {
            size: (half_width, half_height),
            data,
            lines,
            current_line,
        }
    }
}

impl<'a, T> Iterator for WindowIterator<'a, T> {
    /// Returns the current item, its position within the matrix, its position within the window and the window itself.
    type Item = (&'a T, (usize, usize), (usize, usize), Vec<&'a [T]>);

    fn next(&mut self) -> Option<Self::Item> {
        let (col, item) = match self.current_line.as_mut()?.1.next() {
            Some(v) => v,
            None => {
                let (ln, columns) = self.lines.next()?;
                self.current_line = Some((ln, columns.iter().enumerate()));
                self.current_line.as_mut()?.1.next()?
            }
        };
        let ln = self.current_line.as_ref().expect("if `self.current_line` is `None`, previous code should've breaked the control flow").0;

        let (width, height) = self.size;

        let lines_start = ln.saturating_sub(height);
        let lines_end = (ln + height).min(self.data.len() - 1);
        let lines_range = lines_start..=lines_end;

        let columns_start = col.saturating_sub(width);
        let columns_end = (col + width).min(self.data[ln].len() - 1);
        let columns_range = columns_start..=columns_end;

        let window = self.data[lines_range]
            .iter()
            .map(|ln| &ln[columns_range.clone()])
            .collect_vec();

        Some((
            item,
            (col, ln),
            (col - columns_start, ln - lines_start),
            window,
        ))
    }
}

// pub fn iter_2d_window<T>(
//     matrix: &Vec<Vec<T>>,
//     width: usize,
//     height: usize,
//     mut f: impl FnMut(&T, (usize, usize), &[&[T]]),
// ) {
//     let half_width = (width - 1) / 2;
//     let half_height = (height - 1) / 2;
//     for (line_index, line) in matrix.iter().enumerate() {
//         for (column_index, v) in line.iter().enumerate() {
//             let window = neighbours(
//                 matrix.as_slice(),
//                 (line_index, column_index),
//                 (width, height),
//             );
//             // f(v, (line_index, column_index), window);
//         }
//     }
// }

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum Adjacency {
    Adjacent(char),
    Isolated(char),
}

impl Adjacency {
    pub fn is_adjacent(&self) -> bool {
        match self {
            Adjacency::Adjacent(_) => true,
            Adjacency::Isolated(_) => false,
        }
    }

    pub fn char(&self) -> char {
        match self {
            Adjacency::Adjacent(c) => *c,
            Adjacency::Isolated(c) => *c,
        }
    }
}

impl std::fmt::Display for Adjacency {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Adjacency::Adjacent(_) => write!(f, "O"),
            Adjacency::Isolated(_) => write!(f, "-"),
        }
    }
}

type Matrix = Vec<Vec<Adjacency>>;

pub fn part_one(input: &str) -> Option<u32> {
    let matrix = input
        .lines()
        .map(|ln| ln.chars().collect_vec())
        .collect_vec();

    // Check if an item is ajacent to a special character
    let mut adjacency_matrix = WindowIterator::new(&matrix, (3, 3))
        .map(|(v, (_, ln), _, window)| {
            let is_adjacent = window
                .iter()
                .any(|row| row.iter().any(|c| *c != '.' && !c.is_ascii_digit()));

            let adjacency = if is_adjacent {
                Adjacency::Adjacent(*v)
            } else {
                Adjacency::Isolated(*v)
            };

            (adjacency, ln)
        })
        .group_by(|(_, ln)| *ln)
        .into_iter()
        .map(|(_, group)| group.map(|(a, _)| a).collect_vec())
        .collect_vec();

    // Collect numbers
    let mut parts = Vec::<u32>::new();

    for ln in adjacency_matrix {
        let iter = ln.iter();
        let mut data: (bool, u32) = (false, 0);
        for a in iter {
            let c = a.char();

            if c.is_ascii_digit() {
                let n = c.to_digit(10).expect("the digit should always be valid");
                data.0 = data.0 || a.is_adjacent();
                data.1 = data.1 * 10 + n;
                continue;
            }

            if data.0 && data.1 != 0 {
                parts.push(data.1);
            }

            data = (false, 0);
        }

        if data.0 && data.1 != 0 {
            parts.push(data.1);
        }
    }

    Some(parts.iter().sum())
}

pub fn part_two(_input: &str) -> Option<u32> {
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(4361));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}

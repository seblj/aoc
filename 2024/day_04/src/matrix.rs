pub struct Matrix<T> {
    vec: Vec<T>,
    dim: (usize, usize),
}

pub struct MatrixIter<'a, T> {
    matrix: &'a Matrix<T>,
    curr: (i32, i32),
}

#[derive(Debug)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
    UpLeft,
    UpRight,
    DownLeft,
    DownRight,
}

impl Direction {
    pub fn to_index(&self, curr: (i32, i32)) -> (i32, i32) {
        match self {
            Direction::Up => (curr.0, curr.1 - 1),
            Direction::Down => (curr.0, curr.1 + 1),
            Direction::Left => (curr.0 - 1, curr.1),
            Direction::Right => (curr.0 + 1, curr.1),
            Direction::UpLeft => (curr.0 - 1, curr.1 - 1),
            Direction::UpRight => (curr.0 + 1, curr.1 - 1),
            Direction::DownLeft => (curr.0 - 1, curr.1 + 1),
            Direction::DownRight => (curr.0 + 1, curr.1 + 1),
        }
    }
}

impl<T> Matrix<T> {
    pub fn new(vec: Vec<T>, dim: (usize, usize)) -> Self {
        Matrix { dim, vec }
    }

    pub fn iter(&self) -> MatrixIter<T> {
        MatrixIter {
            matrix: self,
            curr: (0, 0),
        }
    }

    pub fn width(&self) -> usize {
        self.dim.0
    }

    pub fn height(&self) -> usize {
        self.dim.1
    }

    pub fn get(&self, pos: (i32, i32)) -> Option<&T> {
        self.in_grid(pos).then(|| &self[pos])
    }

    pub fn iter_adjacent(&self, pos: (usize, usize)) -> impl Iterator<Item = (usize, usize)> + '_ {
        (0..8).filter_map(move |i| {
            let (x, y) = match i {
                0 => (-1, -1),
                1 => (-1, 0),
                2 => (-1, 1),
                3 => (0, 1),
                4 => (1, 1),
                5 => (1, 0),
                6 => (1, -1),
                7 => (0, -1),
                _ => unreachable!(),
            };
            let (w, h) = (pos.0 as i32 + x, pos.1 as i32 + y);
            self.in_grid((w, h)).then_some((w as usize, h as usize))
        })
    }

    pub fn in_grid(&self, idx: (i32, i32)) -> bool {
        idx.0 >= 0
            && (idx.0 as usize) < self.height()
            && idx.1 >= 0
            && (idx.1 as usize) < self.width()
    }
}

impl<'a, T> Iterator for MatrixIter<'a, T> {
    type Item = ((usize, usize), &'a T);

    fn next(&mut self) -> Option<Self::Item> {
        let right = (self.curr.0 + 1, self.curr.1);
        let down = (0, self.curr.1 + 1);

        if self.matrix.in_grid(right) {
            self.curr = right;
            Some(((right.0 as usize, right.1 as usize), &self.matrix[right]))
        } else if self.matrix.in_grid(down) {
            self.curr = down;
            Some(((down.0 as usize, down.1 as usize), &self.matrix[down]))
        } else {
            None
        }
    }
}

impl From<&[String]> for Matrix<u8> {
    fn from(value: &[String]) -> Self {
        Matrix::new(
            value.iter().flat_map(|s| s.bytes()).collect(),
            (value[0].len(), value.len()),
        )
    }
}

macro_rules! impl_index {
    ($typ:ty) => {
        impl<T> std::ops::Index<($typ, $typ)> for Matrix<T> {
            type Output = T;
            fn index(&self, idx: ($typ, $typ)) -> &T {
                assert!(
                    self.in_grid((idx.0 as i32, idx.1 as i32)),
                    "index out of bounds: the dimensions are ({}, {}) but the index is ({}, {})",
                    self.dim.0 - 1,
                    self.dim.1 - 1,
                    idx.0,
                    idx.1
                );
                let idx = (idx.1 as usize * self.dim.0) + idx.0 as usize;
                &self.vec[idx]
            }
        }
    };
}

impl_index!(i32);
impl_index!(usize);

macro_rules! impl_fmt {
    ($typ:ident,$lit:tt) => {
        impl<T: std::fmt::$typ> std::fmt::$typ for Matrix<T> {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                for i in 0..self.vec.len() {
                    if i % self.dim.0 == 0 {
                        writeln!(f).unwrap();
                    }
                    write!(f, $lit, self.vec[i]).unwrap();
                }
                writeln!(f)
            }
        }
    };
}

impl_fmt!(Debug, "{:?}, ");
impl_fmt!(Display, "{}");

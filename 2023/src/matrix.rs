pub struct Matrix<T> {
    vec: Vec<T>,
    dim: (usize, usize),
}

impl<T> Matrix<T> {
    pub fn new(vec: Vec<T>, dim: (usize, usize)) -> Self {
        Matrix { dim, vec }
    }

    pub fn width(&self) -> usize {
        self.dim.0
    }

    pub fn height(&self) -> usize {
        self.dim.1
    }

    pub fn iter_adjacent(&self, w: usize, h: usize) -> impl Iterator<Item = (usize, usize)> + '_ {
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
            let (w, h) = (w as i32 + x, h as i32 + y);
            self.in_grid((w, h)).then_some((w as usize, h as usize))
        })
    }

    fn in_grid(&self, idx: (i32, i32)) -> bool {
        idx.0 >= 0
            && (idx.0 as usize) < self.height()
            && idx.1 >= 0
            && (idx.1 as usize) < self.width()
    }
}

impl<T> std::ops::Index<(usize, usize)> for Matrix<T> {
    type Output = T;
    fn index(&self, idx: (usize, usize)) -> &T {
        assert!(
            self.in_grid((idx.0 as i32, idx.1 as i32)),
            "index out of bounds: the dimensions are ({}, {}) but the index is ({}, {})",
            self.dim.0 - 1,
            self.dim.1 - 1,
            idx.0,
            idx.1
        );
        let idx = (idx.1 * self.dim.0) + idx.0;
        &self.vec[idx]
    }
}

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

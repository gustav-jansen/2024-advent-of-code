#[derive(Debug,PartialEq)]
pub struct Matrix<T> {
    elements: Vec<T>,
    pub nrows: usize,
    pub ncols: usize,
}

impl<T: Clone> Matrix<T> {
    pub fn new(nrows: usize, ncols: usize, initial_value: T) -> Self {
        Matrix {
            elements: vec![initial_value; nrows * ncols],
            nrows,
            ncols,
        }
    }

    pub fn get(&self, row: usize, col: usize) -> Option<&T> {
        if row < self.nrows && col < self.ncols {
            self.elements.get(row * self.ncols + col)
        } else {
            None
        }
    }

    pub fn set(&mut self, row: usize, col: usize, value: T) -> Result<(), &str> {
        if row < self.nrows && col < self.ncols {
            if let Some(elem) = self.elements.get_mut(row * self.ncols + col) {
                *elem = value;
                Ok(())
            } else {
                Err("Index out of bounds")
            }
        } else {
            Err("Index out of bounds")
        }
    }

    pub fn iter(&self) -> impl Iterator<Item=&T> {
        self.elements.iter()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_set_and_get() {
        let mut matrix = Matrix::new(3, 4, ' ');

        matrix.set(1, 2, 'c').expect("Failed to set value");
        let val = matrix.get(1,2).unwrap();
        assert_eq!(*val, 'c');
    }
}

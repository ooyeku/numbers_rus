pub struct Numbers {
    left: f64,
    right: f64,
}

impl Numbers {
    pub fn new(left: f64, right: f64) -> Numbers {
        Numbers { left, right }
    }

    pub fn add(&self) -> f64 {
        self.left + self.right
    }

    // Similar changes for other functions
    // subtract, multiply, divide

    pub fn root(&self) -> f64 {
        let root = 1.0 / self.right;
        self.left.powf(root)
    }

    // and other functions
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_add() {
        let numbers = Numbers::new(1.0, 2.0);
        let result = numbers.add();
        assert_eq!(result, 3.0);
    }

    // Similar tests for other functions
    // subtract, multiply, divide

    #[test]
    fn test_root() {
        let numbers = Numbers::new(4.0, 2.0);
        let result = numbers.root();
        assert_eq!(result, 2.0);
    }

    // and other functions
}

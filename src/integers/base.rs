
    /// `add` function: Takes in two i128 integers `left` and `right` as input and returns their sum as an i128 integer.
    pub fn add(left: i128, right: i128) -> i128 {
        left + right
    }
    /// `subtract` function: Takes two i128 integers `left` and `right` as input and returns their difference as an i128 integer.
    pub fn subtract(left: i128, right: i128) -> i128 {
        left - right
    }
    /// `multiply` function: Takes in two i128 integers `left` and `right` as input and returns their product as an i128 integer.
    pub fn multiply(left: i128, right: i128) -> i128 {
        left * right
    }
    /// `divide` function: Takes in two i128 integers `left` and `right` as input and returns their quotient as an i128 integer.
    pub fn divide(left: i128, right: i128) -> i128 {
        left / right
    }
    /// `modulo` function: Takes two i128 integers `left` and `right` as input and returns their remainder as an i128 integer.
    pub fn modulo(left: i128, right: i128) -> i128 {
        left % right
    }
    /// `power` function: Takes in two i128 integers `left` and `right` as input and returns `left` raised to the power of `right` as an i128 integer.
    /// Only supports non-negative exponent values. Panics if the exponent is negative.
    pub fn power(left: i128, right: i128) -> i128 {
        if right < 0 {
            panic!("Exponentiation with negative exponent is not supported");
        }

        let mut base = left;
        let mut result = 1;

        let mut exponent = right;
        while exponent > 0 {
            if exponent % 2 == 1 {
                result *= base;
            }
            base *= base;
            exponent /= 2;
        }

        result
    }

    /// `root` function: Takes two i128 integers `left` and `right` as input. Returns the `right`th root of `left` as an i128 integer.
    /// Panics if `left` is negative or `right` is non-positive.
    pub fn root(left: i128, right: i128) -> i128 {
        // Check for edge cases
        if left < 0 || right <= 0 {
            panic!("Invalid input: left should be non-negative, and right should be positive");
        }

        // Calculate the root
        let root = 1.0 / right as f64;
        (left as f64).powf(root).round() as i128
    }
    /// `factorial` function: Takes an i128 integer `number` as input and returns its factorial as an i128 integer. The function
    /// uses recursion to calculate the factorial. Note that it may cause stack overflow for large input values. Panics if the input value
    /// is negative. To avoid this behavior, ensure the input value is non-negative.
    pub fn factorial(number: i128) -> i128 {
        if number == 0 {
            1
        } else {
            number * factorial(number - 1)
        }
    }
    /// `fibonacci` function: Takes an i128 integer `number` as input and returns the `number`th Fibonacci number as an i128 integer.
    /// It uses recursion to calculate the Fibonacci number. Note that it may cause stack overflow for large input values.
    pub fn fibonacci(number: i128) -> i128 {
        if number == 0 {
            0
        } else if number == 1 {
            1
        } else {
            fibonacci(number - 1) + fibonacci(number - 2)
        }
    }

    /// `is_prime` function: Takes an i128 integer `number` as input and returns a boolean indicating whether `number` is a prime or not.
    /// The function checks if the input `number` is less than or equal to 1, returning false if true, then iterates through all integers within
    /// the range of 2 to `number` (exclusive), checking if they are divisible by `number`. If a divisor is found, the function returns false.
    /// If no divisor is found, it returns true.
    pub fn is_prime(number: i128) -> bool {
        if number <= 1 {
            return false;
        }

        for i in 2..number {
            if number % i == 0 {
                return false;
            }
        }
        true
    }

    /// `is_even` function: Takes an i128 integer `number` as input and returns a boolean indicating whether `number` is even or not.
    pub fn is_even(number: i128) -> bool {
        number % 2 == 0
    }
    /// `is_odd` function: Takes an i128 integer `number` as input and returns a boolean indicating whether `number` is odd or not.
    pub fn is_odd(number: i128) -> bool {
        number % 2 != 0
    }

    /// `is_perfect_square` function: Takes an i128 integer `number` as input and returns a boolean indicating whether `number`
    /// is a perfect square or not. The function calculates the square root of the input number, rounds it to the nearest integer value, and
    /// then checks if the result squared is equal to the original number. If they are equal, it returns true; otherwise, it returns false.
    pub fn is_perfect_square(number: i128) -> bool {
        let sqrt = (number as f64).sqrt() as i128;
        sqrt * sqrt == number
    }
    /// `is_perfect_cube` function: Takes an i128 integer `number` as input and returns a boolean indicating whether `number`
    /// is a perfect cube or not. The function calculates the cube root of the input number, rounds it to the nearest integer value, and
    /// then checks if the result cubed is equal to the original number. If they are equal, it returns true; otherwise, it returns false.
    pub fn is_perfect_cube(number: i128) -> bool {
        let cbrt = (number as f64).cbrt() as i128;
        cbrt * cbrt * cbrt == number
    }

    /// `is_perfect_power` function: Takes an i128 integer `number` as input and returns a boolean indicating whether
    /// `number` is a perfect power or not. The function calculates the square root of the input number, rounds it to the nearest integer
    /// value, and then checks if the result squared is equal to the original number. If they are equal, it returns true; otherwise, it returns
    /// false.
    pub fn is_perfect_power(number: i128) -> bool {
        let sqrt = (number as f64).sqrt() as i128;
        sqrt * sqrt == number
    }


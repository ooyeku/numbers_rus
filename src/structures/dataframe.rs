//! dataframe.rs contains the dataframe implementation and operations that target the dataframe
//! as a whole (non-element-wise operations).
//! The dataframe structure is similar to pythons pandas dataframe but with less functionality and only for integers.
//! The dataframe is stored as a vector of vectors.

use std::collections::HashMap;

/// A dataframe structure similar to pythons pandas dataframe but with less functionality and only for integers
/// The structures is stored as a HashMap of vectors
///
/// # Example
/// ```
/// use numbers_rus::structures::dataframe::DataFrame;
/// let mut df = DataFrame::new();
/// df.add_column("a", &vec![1, 2, 3]);
/// df.add_column("b", &vec![4, 5, 6]);
/// df.add_column("c", &vec![7, 8, 9]);
/// ```
///
pub struct DataFrame {
    data: HashMap<String, Vec<i128>>,
}

impl DataFrame {
    /// Creates a new dataframe
    pub fn new() -> DataFrame {
        DataFrame {
            data: HashMap::new(),
        }
    }

    /// Adds a column to the dataframe
    pub fn add_column(&mut self, name: &str, data: &Vec<i128>) {
        self.data.insert(name.to_string(), data.clone());
    }

    /// Returns the column names
    pub fn get_column_names(&self) -> Vec<String> {
        self.data.keys().cloned().collect()
    }

    /// Returns the data
    pub fn get_data(&self) -> &HashMap<String, Vec<i128>> {
        &self.data
    }

    /// Returns the data for a column
    pub fn get_column(&self, name: &str) -> Result<Vec<i128>, &'static str> {
        if self.column_exists(name) {
            Ok(self.data.get(name).unwrap().clone())
        } else {
            Err("Column not found")
        }
    }

    /// Returns the number of columns
    pub fn get_column_count(&self) -> usize {
        self.data.keys().len()
    }

    /// Returns the number of rows
    pub fn get_row_count(&self) -> usize {
        self.data.values().next().map(|v| v.len()).unwrap_or(0)
    }

    /// check if column exists
    fn column_exists(&self, name: &str) -> bool {
        self.data.contains_key(name)
    }
}

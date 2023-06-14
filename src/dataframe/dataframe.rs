pub mod dataframe {
    // Dataframe functions  ----------------------------------------------------------------------------

/// A dataframe struct similar to pythons pandas dataframe but with less functionality and only for integers
/// The dataframe is stored as a vector of vectors
/// The first vector contains the column names
/// The rest of the vectors contain the data
/// The data is stored as a vector of vectors
///
/// # Example
/// ```
/// use numbers_rus::dataframe::dataframe::dataframe::DataFrame;
/// let mut df = DataFrame::new();
/// df.add_column("a", &vec![1, 2, 3]);
/// df.add_column("b", &vec![4, 5, 6]);
/// df.add_column("c", &vec![7, 8, 9]);
/// ```
///
pub struct DataFrame {
        columns: Vec<String>,
    data: Vec<Vec<i128>>,
}
impl DataFrame {
        /// Creates a new dataframe
    pub fn new() -> DataFrame {
            DataFrame {
                columns: Vec::new(),
                data: Vec::new(),
            }
        }
    /// Adds a column to the dataframe
    pub fn add_column(&mut self, name: &str, data: &Vec<i128>) {
            self.columns.push(name.to_string());
            self.data.push(data.clone());
        }
    /// Returns the column names
    pub fn get_columns(&self) -> Vec<String> {
            self.columns.clone()
        }
    /// Returns the data
    pub fn get_data(&self) -> Vec<Vec<i128>> {
            self.data.clone()
        }
    /// Returns the column names
    pub fn get_column(&self, name: &str) -> Result<Vec<i128>, &'static str> {
            let mut result = Vec::new();

            for i in 0..self.columns.len() {
                if self.columns[i] == name {
                    result = self.data[i].clone();
                }
            }
        if result.len() == 0 {
            return Err("Column not found");
        }
        Ok(result)
        }
    /// Returns the column names
    pub fn get_column_index(&self, name: &str) -> Result<i128, &'static str> {
            let mut result = 0;

            for i in 0..self.columns.len() {
                if self.columns[i] == name {
                    result = i;
                }
            }

        if result == 0 {
            return Err("Column not found");
        }

        Ok(result as i128)
        }
    /// Returns the column names
    pub fn get_column_name(&self, index: i128) -> Result<String, &'static str> {
            if index > self.columns.len() as i128 {
                return Err("Column not found");
            }

        Ok(self.columns[index as usize].clone())
        }
    /// Returns the column names
    pub fn get_column_count(&self) -> i128 {
            self.columns.len() as i128
        }
    /// Returns the column names
    pub fn get_row_count(&self) -> usize {
            self.data[0].len()
        }
    }

/// Creates a new dataframe
pub fn dataframe_create(
    columns: Vec<String>,
    data: Vec<Vec<i128>>,
    ) -> Result<DataFrame, &'static str> {
        if columns.len() != data.len() {
            return Err("Column count does not match data count");
        }

    for i in 0..data.len() {
        if data[i].len() != data[0].len() {
            return Err("Data length does not match");
        }
    }

    Ok(DataFrame {
        columns,
        data,
    })
    }
#[cfg(test)]
mod test_dataframe_create {
        use super::*;

        #[test]
    fn it_works() {
            let result = dataframe_create(
                vec!["a".to_string(), "b".to_string(), "c".to_string()],
            vec![vec![1, 2, 3], vec![1, 2, 3], vec![1, 2, 3]],
        );

            assert_eq!(result.is_ok(), true);
        }
    #[test]
    fn display_dataframe() {
            let result = dataframe_create(
                vec!["a".to_string(), "b".to_string(), "c".to_string()],
            vec![vec![1, 2, 3], vec![1, 2, 3], vec![1, 2, 3]],
        );

            assert_eq!(
                result.unwrap().get_columns(),
            vec!["a".to_string(), "b".to_string(), "c".to_string()]
        );
        }
    }
}
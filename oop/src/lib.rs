pub struct AveragedCollection {
    list: Vec<i32>,
    average: f64,
}

impl AveragedCollection {
    // pub fn new() -> AveragedCollection {
    //     self.list = vec![];
    //     self.average = 0;
    // }

    pub fn add(&mut self, value: i32) {
        self.list.push(value);
        self.update_average();
    }

    pub fn remove(&mut self) -> Option<i32> {
        let result = self.list.pop();
        match result {
            Some(value) => {
                self.update_average();
                Some(value)
            }
            None => None,
        }
    }

    pub fn average(&self) -> f64 {
        self.average
    }

    fn update_average(&mut self) {
        let total: i32 = self.list.iter().sum();
        self.average = total as f64 / self.list.len() as f64;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_first_insertion() {
        let mut averaged_collection = AveragedCollection {
            list: vec![],
            average: 0.0,
        };
        averaged_collection.add(5);
        assert_eq!(averaged_collection.average, 5.0);
    }
    #[test]
    fn test_two_insertions() {
        let mut averaged_collection = AveragedCollection {
            list: vec![],
            average: 0.0,
        };
        averaged_collection.add(5);
        averaged_collection.add(0);
        assert_eq!(averaged_collection.average, 2.5);
    }
    #[test]
    fn test_three_insertions() {
        let mut averaged_collection = AveragedCollection {
            list: vec![],
            average: 0.0,
        };
        averaged_collection.add(5);
        averaged_collection.add(0);
        averaged_collection.add(10);
        assert_eq!(averaged_collection.average, 5.0);
    }
}

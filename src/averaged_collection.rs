pub struct AveragedCollection {
    values: Vec<i32>,
    average: f64,
}

impl AveragedCollection {
    pub fn new(values: Vec<i32>) -> Self {
        let mut new_coll = Self {
            values,
            average: 0.0,
        };
        new_coll.update_average();
        new_coll
    }

    pub fn add(&mut self, value: i32) {
        self.values.push(value);
        self.update_average();
    }

    pub fn remove(&mut self) -> Option<i32> {
        let result = self.values.pop();

        // Option::inspect()
        // fn inspect<F: FnOnce(&T)>(self, f: F) -> Self
        result.inspect(|val| { self.update_average(); })
        // or
        // result.map(|val| {
        //     self.update_average();
        //     val
        // })
        // or
        // match result {
        //   Some(value) => {
        //     self.update_average();
        //     Some(value)
        //   },
        //   None => None,
        // }
    }

    pub fn average(&self) -> f64 {
        self.average
    }

    fn update_average(&mut self) {
        let sum: i32 = self.values.iter().sum();
        self.average = sum as f64 / self.values.len() as f64;
    }
}

#[test]
fn ex1_avarage_basic() {
    let coll = AveragedCollection::new(vec![1, 3]);
    assert_eq!(coll.average(), 2.0);
}

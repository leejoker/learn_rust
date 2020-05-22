pub struct AveragedCollection {
    list: Vec<i32>,
    average: f64,
}
impl AveragedCollection {
    pub fn new() -> AveragedCollection {
        AveragedCollection {
            list: vec![],
            average: 0.0,
        }
    }
    pub fn add(&mut self, value: i32) {
        self.list.push(value);
        self.update_average();
    }
    pub fn remove(&mut self) -> Option<i32> {
        let result = self.list.pop();
        match result {
            Some(v) => {
                self.update_average();
                Some(v)
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

#[#[cfg(test)]]
mod test{
    #[test]
    fn run_test(){
        let mut average_collection = AveragedCollection::new();
        average_collection.add(5);
        average_collection.add(10);
        println!("average is {}", average_collection.average());
        let remove_val = average_collection.remove();
        println!("removed value is {}", remove_val.unwrap());
        println!("after remove average is {}", average_collection.average());
    }
}

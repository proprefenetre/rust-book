// Given a list of integers, use a vector and return the mean (average), median
// (when sorted, the value in the middle position), and mode (the value that occurs
// most often (https://doc.rust-lang.org/book/second-edition/ch08-03-hash-maps.html).

use std::collections::HashMap;

#[derive(Debug)]
pub struct CentralTendencies {
    pub mean: f32,
    pub median: f32,
    pub mode: Vec<i32>,
}

impl CentralTendencies {
    pub fn new() -> CentralTendencies {
        CentralTendencies { mean: 0f32, median: 0f32, mode: vec![0],}
    }

    pub fn calculate(&mut self, data: Vec<i32>) {
        self.mean = self.calc_mean(&data);
        self.mode = self.calc_mode(&data);
        self.median = self.calc_median(data);
    }

    // calculate mean
    fn calc_mean(&self, data: &[i32]) -> f32 {
        data.iter().fold(0i32, |s, &n| s + n as i32) as f32 / data.len() as f32
    }

    // median middle value of a sorted vector, or the mean of the two center 
    // values if the length of the vector is an even number
    fn calc_median(&self, mut data: Vec<i32>) -> f32 {
        data.sort();
        let mid = data.len() / 2;
        if data.len() % 2 == 0 {
            (data[mid-1] as f32 + data[mid] as f32) / 2 as f32
        } else {
            data[mid] as f32
        }
    }
    
    // mode: the number(s) that occurs most frequently
    fn calc_mode(&self, data: &[i32]) -> Vec<i32> {
        let mut map: HashMap<i32, i32> = HashMap::new();
        for n in data {
            *map.entry(*n).or_insert(0) += 1;
        }

        let max = map.values()
                     .max()
                     .unwrap()
                     .clone();

        map.into_iter()
            .filter(|v| v.1 == max)
            .map(|v| v.0)
            .collect::<Vec<i32>>()
        
    }
}

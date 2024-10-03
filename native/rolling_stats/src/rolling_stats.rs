use crate::sortable::Sortable;
use crate::stats_data::StatsData;
use std::collections::{BTreeMap, VecDeque};

struct MinMaxIndex {
    items: BTreeMap<Sortable<f64>, u32>,
}

impl MinMaxIndex {
    fn new() -> Self {
        Self {
            items: BTreeMap::new(),
        }
    }

    fn insert_item(&mut self, key: f64) {
        *self.items.entry(Sortable(key)).or_insert(0) += 1;
    }

    fn remove_item(&mut self, key: f64) {
        if let Some(count) = self.items.get_mut(&Sortable(key)) {
            if *count > 1 {
                *count -= 1;
            } else {
                self.items.remove(&Sortable(key));
            }
        }
    }

    fn get_min(&self) -> Option<f64> {
        self.items.keys().next().map(|value| value.0)
    }

    fn get_max(&self) -> Option<f64> {
        self.items.keys().next_back().map(|value| value.0)
    }
}

pub struct RollingStats {
    queue: VecDeque<f64>,
    max_size: u32,
    sum: f64,
    sum_squares: f64,
    index: MinMaxIndex,
}

impl RollingStats {
    pub fn new(max_size: u32) -> Self {
        Self {
            max_size,
            index: MinMaxIndex::new(),
            queue: VecDeque::new(),
            sum: 0.0,
            sum_squares: 0.0,
        }
    }

    pub fn add_items(&mut self, items: Vec<f64>) {
        for item in items {
            if self.count() >= self.max_size {
                let popped_value = self.queue.pop_front().unwrap();
                self.sum -= popped_value;
                self.sum_squares -= popped_value.powi(2);
                self.index.remove_item(popped_value);
            }
            self.queue.push_back(item);
            self.sum += item;
            self.sum_squares += item.powi(2);
            self.index.insert_item(item);
        }
    }

    fn count(&self) -> u32 {
        self.queue.len() as u32
    }

    fn average(&self) -> f64 {
        self.sum / self.count() as f64
    }

    fn variance(&self) -> f64 {
        (self.sum_squares - (self.sum.powi(2) / self.count() as f64)) / self.count() as f64
    }

    pub fn get_stats(&self) -> Option<StatsData> {
        if self.count() == 0 {
            None
        } else {
            Some(StatsData {
                average: self.average(),
                variance: self.variance(),
                last: *self.queue.back()?,
                min: self.index.get_min()?,
                max: self.index.get_max()?,
            })
        }
    }
}

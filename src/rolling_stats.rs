use std::collections::{BTreeMap, VecDeque};
use crate::sortable_f32::SortableF32;

struct MinMaxIndex {
    items: BTreeMap<SortableF32, u32>,
}

impl MinMaxIndex {
    fn new() -> Self {
        Self { items: BTreeMap::new() }
    }

    fn insert_item(&mut self, key: f32) {
        *self.items.entry(SortableF32(key)).or_insert(0) += 1;
    }

    fn remove_item(&mut self, key: f32) {
        if let Some(count) = self.items.get_mut(&SortableF32(key)) {
            if *count > 1 {
                *count -= 1;
            } else {
                self.items.remove(&SortableF32(key));
            }
        }
    }

    fn get_min(&self) -> Option<f32> {
        self.items.keys().next().map(|value| value.0)
    }

    fn get_max(&self) -> Option<f32> {
        self.items.keys().next_back().map(|value| value.0)
    }
}

#[derive(Debug)]
pub struct StatsData {
    average: f32,
    variance: f32,
    last: f32,
    min: f32,
    max: f32,
}

pub struct RollingStats {
    queue: VecDeque<f32>,
    max_size: u32,
    sum: f32,
    sum_squares: f32,
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

    pub fn add_items(&mut self, items: Vec<f32>) {
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

    fn average(&self) -> f32 {
        self.sum / self.count() as f32
    }

    fn variance(&self) -> f32 {
        (self.sum_squares - (self.sum.powi(2) / self.count() as f32)) / self.count() as f32
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

use std::cmp::Ordering;
use std::collections::VecDeque;
use std::collections::BTreeMap;

fn main() {
    let mut c = MainCollection::new(10);
    c.add_items(vec![1.0, 2.0, 3.0, 4.0, 5.0]);
    c.add_items(vec![6.0, 7.0, 8.0, 9.0, 10.0]);
    c.add_items(vec![11.0, 12.0, 13.0, 14.0, 15.0]);
    let data = c.get_data();
    println!("{:?}", data);
    println!("{:?}", c.queue);
}

#[derive(Debug, PartialEq)]
struct TotalF32(f32);

impl Eq for TotalF32 {}

impl PartialOrd for TotalF32 {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for TotalF32 {
    fn cmp(&self, other: &Self) -> Ordering {
        self.0.partial_cmp(&other.0).unwrap_or(Ordering::Equal)
    }
}

struct MinMaxIndex {
    items: BTreeMap<TotalF32, u32>,
}

impl MinMaxIndex {
    fn new() -> Self {
        MinMaxIndex {
            items: BTreeMap::new(),
        }
    }

    fn insert_item(&mut self, key: f32) {
        let count = self.items.entry(TotalF32(key)).or_insert(0);
        *count += 1;
    }

    fn remove_item(&mut self, key: f32) {
        let entry = self.items.entry(TotalF32(key));
        if let std::collections::btree_map::Entry::Occupied(mut occupied_entry) = entry {
            let count = occupied_entry.get_mut();
            if *count > 1 {
                *count -= 1;
            } else {
                occupied_entry.remove();
            }
        }
    }

    fn get_min(&self) -> Option<f32> {
        self.items.keys().next().map(|total_f32| total_f32.0)
    }

    fn get_max(&self) -> Option<f32> {
        self.items.keys().next_back().map(|total_f32| total_f32.0)
    }
}

#[derive(Debug)]
struct Data {
    average: f32,
    variance: f32,
    last: f32,
    min: f32,
    max: f32,
}

struct MainCollection {
    queue: VecDeque<f32>,
    max_size: u32,
    sum: f32,
    sum_squares: f32,
    count: u32,
    index: MinMaxIndex,
}

impl MainCollection {
    fn new(max_size: u32) -> Self {
        MainCollection {
            queue: VecDeque::new(),
            max_size,
            sum: 0.0,
            sum_squares: 0.0,
            count: 0,
            index: MinMaxIndex::new(),
        }
    }

    fn add_items(&mut self, items: Vec<f32>) {
        for item in items {
            let mut sum_delta = 0.0;
            let mut sum_squares_delta = 0.0;
            if self.queue.len() >= self.max_size as usize {
                let popped_value = self.queue.pop_front().unwrap();
                sum_delta = sum_delta - popped_value;
                sum_squares_delta = sum_squares_delta - popped_value.powi(2);
                self.count -= 1;
                self.index.remove_item(popped_value);
            }
            sum_delta = sum_delta + item;
            sum_squares_delta = sum_squares_delta + item.powi(2);
            self.queue.push_back(item);
            self.sum = self.sum + sum_delta;
            self.sum_squares = self.sum_squares + sum_squares_delta;
            self.count += 1;
            self.index.insert_item(item);
        }
    }

    fn average(&self) -> Option<f32> {
        if self.count == 0 {
            None
        } else {
            Some(self.sum / self.count as f32)
        }
    }

    fn variance(&self) -> Option<f32> {
        if self.count < 2 {
            None
        } else {
            Some((self.sum_squares - (self.sum.powi(2) / self.count as f32)) / (self.count as f32))
        }
    }

    fn last(&self) -> Option<f32> {
        self.queue.back().copied()
    }

    fn min(&self) -> Option<f32> {
        self.index.get_min()
    }

    fn max(&self) -> Option<f32> {
        self.index.get_max()
    }

    fn get_data(&self) -> Option<Data> {
        if self.count == 0 {
            None
        } else {
            Some(Data {
                average: self.average()?,
                variance: self.variance()?,
                last: self.last()?,
                min: self.min()?,
                max: self.max()?,
            })
        }
    }
}

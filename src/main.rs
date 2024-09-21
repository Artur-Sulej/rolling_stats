use std::collections::VecDeque;

fn main() {
    //
}

struct MainCollection {
    queue: VecDeque<f32>,
    max_size: u32,
    sum: f32,
    sum_squares: f32,
    count: u32,
}

impl MainCollection {
    fn new(max_size: u32) -> Self {
        MainCollection {
            queue: VecDeque::new(),
            max_size,
            sum: 0.0,
            sum_squares: 0.0,
            count: 0,
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
            }
            sum_delta = sum_delta + item;
            sum_squares_delta = sum_delta + item.powi(2);
            self.queue.push_back(item);
            self.sum = self.sum + sum_delta;
            self.count += 1;
        }
    }

    fn average(&self) -> f32 {
        self.sum / self.count as f32
    }

    fn variance(&self) -> f32 {
        self.sum_squares / self.count as f32 - self.average().powi(2)
    }

    fn last(&self) -> Option<f32> {
        self.queue.back().copied()
    }
}

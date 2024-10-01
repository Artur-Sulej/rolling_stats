use std::collections::HashMap;
use crate::rolling_stats::{RollingStats, StatsData};

pub(crate) struct MultiRollingStats {
    stats_map: HashMap<u8, RollingStats>,
    max_exponent: u8,
}

impl MultiRollingStats {
    pub(crate) fn new(max_exponent: u8) -> Self {
        let stats_map: HashMap<u8, RollingStats> =
            (1..=max_exponent).map(|exponent| (exponent, RollingStats::new((10.0f64).powi(exponent as i32) as u32))).collect();

        Self {
            stats_map,
            max_exponent,
        }
    }

    pub(crate) fn add_items(&mut self, items: Vec<f64>) {
        (1..=self.max_exponent).for_each(|exponent| {
            self.stats_map.entry(exponent).and_modify(|stats| stats.add_items(items.clone()));
        });
    }

    pub(crate) fn get_stats(&self, exponent: u8) -> Option<StatsData> {
        if exponent > self.max_exponent || exponent < 1 {
            return None;
        }
        self.stats_map.get(&exponent)?.get_stats()
    }
}

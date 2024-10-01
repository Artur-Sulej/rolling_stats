use std::collections::HashMap;
use crate::multi_rolling_stats::MultiRollingStats;
use crate::rolling_stats::StatsData;

pub(crate) struct AllStats {
    stats_map: HashMap<char, MultiRollingStats>,
}

impl AllStats {
    pub(crate) fn new() -> Self {
        Self { stats_map: HashMap::new() }
    }

    pub(crate) fn add_items(&mut self, identifier: char, items: Vec<f64>) {
        const MAX_EXPONENT: u8 = 8;
        self.stats_map.entry(identifier)
            .or_insert_with(|| MultiRollingStats::new(MAX_EXPONENT))
            .add_items(items);
    }

    pub(crate) fn get_stats(&self, identifier: char, exponent: u8) -> Option<StatsData> {
        self.stats_map.get(&identifier)?.get_stats(exponent)
    }
}

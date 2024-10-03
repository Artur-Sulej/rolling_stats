use dashmap::DashMap;
use std::sync::LazyLock;

mod multi_rolling_stats;
mod rolling_stats;
mod sortable;
mod stats_data;

use crate::multi_rolling_stats::MultiRollingStats;
use crate::stats_data::StatsData;

static mut ALL_STATS: LazyLock<DashMap<String, MultiRollingStats>> =
    LazyLock::new(|| DashMap::new());

#[rustler::nif]
fn add_items(identifier: String, items: Vec<f64>) {
    const MAX_EXPONENT: u8 = 8;
    unsafe {
        ALL_STATS
            .entry(identifier)
            .or_insert_with(|| MultiRollingStats::new(MAX_EXPONENT))
            .add_items(items);
    }
}

#[rustler::nif]
fn get_stats(identifier: String, exponent: u8) -> Option<StatsData> {
    unsafe { ALL_STATS.get(&identifier)?.get_stats(exponent) }
}

rustler::init!("Elixir.RollingStats.Native");

mod sortable_f32;
mod rolling_stats;

use rolling_stats::RollingStats;

fn main() {
    let mut rolling_stats = RollingStats::new(10);
    println!("--- rolling_stats.get_stats() {:?}", rolling_stats.get_stats());
    rolling_stats.add_items(vec![1.0, 2.0, 3.0, 4.0, 5.0]);
    rolling_stats.add_items(vec![6.0, 7.0, 8.0, 9.0, 10.0]);
    rolling_stats.add_items(vec![11.0, 12.0, 13.0, 14.0, 15.0]);
    let stats_data = rolling_stats.get_stats();
    println!("{:?}", stats_data);
}

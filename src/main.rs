mod sortable;
mod rolling_stats;
mod multi_rolling_stats;
mod all_stats;

use rolling_stats::RollingStats;
use crate::all_stats::AllStats;

fn main() {
    let mut rolling_stats = RollingStats::new(10);
    println!("--- rolling_stats.get_stats() {:?}", rolling_stats.get_stats());
    rolling_stats.add_items(vec![1.0, 2.0, 3.0, 4.0, 5.0]);
    rolling_stats.add_items(vec![6.0, 7.0, 8.0, 9.0, 10.0]);
    rolling_stats.add_items(vec![11.0, 12.0, 13.0, 14.0, 15.0]);
    let stats_data = rolling_stats.get_stats();
    println!("{:?}", stats_data);

    let mut all_stats = AllStats::new();
    all_stats.add_items('a', vec![1.4, 2.6, 3.0, 4.2, 5.1]);
    all_stats.add_items('a', vec![6.4, 7.6, 8.0, 9.2, 10.1]);
    all_stats.add_items('a', vec![11.4, 12.6, 13.0, 14.2, 15.1]);
    all_stats.add_items('a', vec![11.4, 12.6, 13.0, 14.2, 15.1]);
    all_stats.add_items('a', vec![11.4, 12.6, 13.0, 14.2, 15.1]);

    all_stats.add_items('b', vec![11.4, 12.6, 13.0, 14.2, 15.1]);
    all_stats.add_items('b', vec![11.4, 444.6, 13.0, 555.2, 777.777]);

    all_stats.add_items('c', vec![0.1, 0.5, 0.9]);

    let stats_a1 = all_stats.get_stats('a', 1);
    let stats_a2 = all_stats.get_stats('a', 2);
    let stats_a3 = all_stats.get_stats('a', 3);

    let stats_b1 = all_stats.get_stats('b', 1);
    let stats_b2 = all_stats.get_stats('b', 2);
    let stats_b3 = all_stats.get_stats('b', 3);

    let stats_c1 = all_stats.get_stats('c', 1);
    let stats_c2 = all_stats.get_stats('c', 2);
    let stats_c3 = all_stats.get_stats('c', 3);

    println!("stats_a1: {:?}", stats_a1);
    println!("stats_a2: {:?}", stats_a2);
    println!("stats_a3: {:?}", stats_a3);

    println!("stats_b1: {:?}", stats_b1);
    println!("stats_b2: {:?}", stats_b2);
    println!("stats_b3: {:?}", stats_b3);

    println!("stats_c1: {:?}", stats_c1);
    println!("stats_c2: {:?}", stats_c2);
    println!("stats_c3: {:?}", stats_c3);
}

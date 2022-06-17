use std::cmp::Ordering;

use r2r::builtin_interfaces::msg::{Duration, Time};
use r2r::geometry_msgs::msg::TransformStamped;

pub fn get_nanos_from_duration(dur: &Duration) -> i64 {
    i64::from(dur.sec) * 1_000_000_000 + i64::from(dur.nanosec)
}

pub fn get_nanos_from_time(t: &Time) -> i64 {
    i64::from(t.sec) * 1_000_000_000 + i64::from(t.nanosec)
}

#[derive(Clone, Debug)]
pub(crate) struct OrderedTF {
    pub(crate) tf: TransformStamped,
}

impl PartialEq for OrderedTF {
    fn eq(&self, other: &Self) -> bool {
        get_nanos_from_time(&self.tf.header.stamp) == get_nanos_from_time(&other.tf.header.stamp)
    }
}

impl Eq for OrderedTF {}

impl Ord for OrderedTF {
    fn cmp(&self, other: &Self) -> Ordering {
        get_nanos_from_time(&self.tf.header.stamp).cmp(
            &get_nanos_from_time(&other.tf.header.stamp))
    }
}

impl PartialOrd for OrderedTF {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(get_nanos_from_time(&self.tf.header.stamp).cmp(
            &get_nanos_from_time(&other.tf.header.stamp)))
    }
}

// use r2r::builtin_interfaces::msg::Time;
// use std::time::Duration;
use r2r::builtin_interfaces::msg::{Duration, Time};
use r2r::geometry_msgs::msg::TransformStamped; 
use r2r::std_msgs::msg::Header; 

use crate::{
    ordered_tf::{OrderedTF, get_nanos_from_duration, get_nanos_from_time},
    tf_error::TfError,
    transforms::{
        interpolate, 
        to_transform_stamped,
    },
};

#[derive(Clone, Debug)]
pub(crate) struct TfIndividualTransformChain {
    cache_duration: Duration,
    static_tf: bool,
    //TODO:  Implement a circular buffer. Current method is slowww.
    pub(crate) transform_chain: Vec<OrderedTF>,
    latest_stamp: Time,
}

impl TfIndividualTransformChain {
    pub fn new(static_tf: bool, cache_duration: Duration) -> Self {
        Self {
            cache_duration,
            transform_chain: Vec::new(),
            static_tf,
            latest_stamp: Time { sec: 0, nanosec: 0 },
        }
    }

    pub fn add_to_buffer(&mut self, msg: TransformStamped) {
        if get_nanos_from_time(&msg.header.stamp) > get_nanos_from_time(&self.latest_stamp) {
            self.latest_stamp = msg.header.stamp.clone();
        }
        let res = self
            .transform_chain
            .binary_search(&OrderedTF { tf: msg.clone() });

        match res {
            Ok(x) => self.transform_chain.insert(x, OrderedTF { tf: msg }),
            Err(x) => self.transform_chain.insert(x, OrderedTF { tf: msg }),
        }

        let time_to_keep = if get_nanos_from_time(&self.latest_stamp) > get_nanos_from_duration(&self.cache_duration) {
            Duration { sec: self.latest_stamp.sec - self.cache_duration.sec, nanosec: self.latest_stamp.nanosec - self.cache_duration.nanosec }
        } else {
            Duration { sec: 0, nanosec: 0 }
        };
        while !self.transform_chain.is_empty() {
            if let Some(first) = self.transform_chain.first() {
                if get_nanos_from_time(&first.tf.header.stamp) < get_nanos_from_duration(&time_to_keep) {
                    self.transform_chain.remove(0);
                } else {
                    break;
                }
            }
        }
    }

    pub fn get_closest_transform(&self, time: Time) -> Result<TransformStamped, TfError> {
        if self.static_tf {
            return Ok(self.transform_chain.last().unwrap().tf.clone());
        }

        let mut res = TransformStamped::default();
        res.header.stamp = time.clone();
        res.transform.rotation.w = 1f64;

        let res = self.transform_chain.binary_search(&OrderedTF { tf: res });

        match res {
            Ok(x) => return Ok(self.transform_chain.get(x).unwrap().tf.clone()),
            Err(x) => {
                if x == 0 {
                    return Err(TfError::AttemptedLookupInPast(
                        time,
                        self.transform_chain.first().unwrap().tf.clone(),
                    ));
                }
                if x >= self.transform_chain.len() {
                    return Err(TfError::AttemptedLookUpInFuture(
                        self.transform_chain.last().unwrap().tf.clone(),
                        time,
                    ));
                }
                let tf1 = self
                    .transform_chain
                    .get(x - 1)
                    .unwrap()
                    .clone()
                    .tf
                    .transform;
                let tf2 = self.transform_chain.get(x).unwrap().clone().tf.transform;
                let time1 = self.transform_chain.get(x - 1).unwrap().tf.header.stamp.clone();
                let time2 = self.transform_chain.get(x).unwrap().tf.header.stamp.clone();
                let header = self.transform_chain.get(x).unwrap().tf.header.clone();
                let child_frame = self
                    .transform_chain
                    .get(x)
                    .unwrap()
                    .tf
                    .child_frame_id
                    .clone();
                let total_duration = Duration{ sec: time2.sec - time1.sec, nanosec: 0 };
                let desired_duration = Duration{ sec: time.sec - time1.sec, nanosec: 0 };
                let weight = 1.0 - 
                    get_nanos_from_duration(&desired_duration) as f64 / 
                    get_nanos_from_duration(&total_duration) as f64;
                let final_tf = interpolate(tf1, tf2, weight);
                let ros_msg = to_transform_stamped(final_tf, header.frame_id, child_frame, time);
                Ok(ros_msg)
            }
        }
    }

    pub fn has_valid_transform(&self, time: Time) -> bool {
        if self.static_tf {
            return true;
        }
        !matches!(self.transform_chain.binary_search(&OrderedTF {
            tf: TransformStamped {
                header: Header {
                    stamp: time,
                    ..Default::default()
                },
                ..Default::default()
            },
        }), Err(x) if x == 0 || x >= self.transform_chain.len())
    }
}

use std::sync::{Arc, RwLock};

use crate::{
    tf_buffer::TfBuffer,
    tf_error::TfError,
};

use r2r::geometry_msgs::msg::TransformStamped; 
use r2r::tf2_msgs::msg::TFMessage;
use r2r::builtin_interfaces::msg::Time;

///This struct tries to be the same as the C++ version of `TransformListener`. Use this struct to lookup transforms.
/// Do note that unlike the C++ variant of the TfListener, only one TfListener can be created at a time. Like its C++ counterpart,
/// it must be scoped to exist through the lifetime of the program. One way to do this is using an `Arc` or `RwLock`.
pub struct TfListener {
    buffer: Arc<RwLock<TfBuffer>>,
}

impl TfListener {
    /// Create a new TfListener
    pub fn new() -> Self {
        Self::new_with_buffer(TfBuffer::new())
    }

    pub fn new_with_buffer(tf_buffer: TfBuffer) -> Self {
        let buff = RwLock::new(tf_buffer);
        let arc = Arc::new(buff);
        TfListener {
            buffer: arc,
        }
    }

    /// Looks up a transform within the tree at a given time.
    pub fn lookup_transform(
        &self,
        from: &str,
        to: &str,
        time: Time,
        msg: TFMessage, 
    ) -> Result<TransformStamped, TfError> {
        self.buffer.write().unwrap().handle_incoming_transforms(msg, false);
        self.buffer.read().unwrap().lookup_transform(from, to, time)
    }
}

impl Default for TfListener {
    fn default() -> Self {
        TfListener::new()
    }
}

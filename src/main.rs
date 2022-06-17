use tf_rosrust::TfListener;
use futures::future;
use futures::stream::StreamExt;
use r2r::QosProfile;
// use r2r::builtin_interfaces::msg::{Duration, Time};
use r2r::builtin_interfaces::msg::Time;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let ctx = r2r::Context::create()?;
    let mut node = r2r::Node::create(ctx, "listener_node", "")?;
    let listener = TfListener::new();

    let sub = node.subscribe::<r2r::tf2_msgs::msg::TFMessage>("/tf", QosProfile::default())?;

    let handle = tokio::task::spawn_blocking(move || loop {
        node.spin_once(std::time::Duration::from_millis(100));
    });

    sub.for_each(|msg| {
        let t = Time {sec: 1645594958, nanosec: 627320528}; 
        let tf = listener.lookup_transform("base_link", "odom", t, msg);
        match tf {
            Ok(pose) => println!("pose {:?}", pose), 
            Err(_e) => {},
        }
        future::ready(())
    })
    .await;

    handle.await?;

    Ok(())
}
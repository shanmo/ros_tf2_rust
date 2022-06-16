use tf_rosrust::TfListener;
use futures::future;
use futures::stream::StreamExt;
use r2r::QosProfile;
use r2r::builtin_interfaces::msg::{Duration, Time};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let ctx = r2r::Context::create()?;
    let mut node = r2r::Node::create(ctx, "listener_node", "")?;
    let listener = TfListener::new();

    let mut sub = node.subscribe::<r2r::std_msgs::msg::String>("/tf", QosProfile::default())?;

    let handle = tokio::task::spawn_blocking(move || loop {
        node.spin_once(std::time::Duration::from_millis(100));
    });

    sub.for_each(|msg| {
        let tf = listener.lookup_transform("camera", "base_link", Time::new(), msg);
        println!("{tf:?}");
        future::ready(())
    })
    .await;

    handle.await?;

    Ok(())
}
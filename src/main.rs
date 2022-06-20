use tf_rosrust::TfListener;
use futures::future;
use futures::stream::StreamExt;
use r2r::QosProfile;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let ctx = r2r::Context::create()?;
    let mut node = r2r::Node::create(ctx, "listener_node", "")?;
    let listener = TfListener::new();

    let sub = node.subscribe::<r2r::tf2_msgs::msg::TFMessage>("/tf", QosProfile::default())?;

    let handle = tokio::task::spawn_blocking(move || loop {
        node.spin_once(std::time::Duration::from_millis(100));
    });

    // let mut clock = r2r::Clock::create(r2r::ClockType::RosTime)?;

    sub.for_each(|msg| {
        // let now = clock.get_now().unwrap();
        // let time = r2r::Clock::to_builtin_time(&now);
        // println!("rostime: {:?}", time);
        // println!("msg time: {:?}", msg.transforms[0].header.stamp);
        let mut time = msg.transforms[0].header.stamp.clone();
        time.nanosec -= 1; 
        let tf = listener.lookup_transform("base_link", "odom", time, msg);
        match tf {
            Ok(pose) => println!("interpolated pose\n{:?}", pose), 
            Err(_e) => {},
        }
        future::ready(())
    })
    .await;

    handle.await?;

    Ok(())
}
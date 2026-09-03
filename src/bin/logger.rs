use rclrs::*;
use sensor_msgs::msg::Image as RosImage;
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let context = Context::default_from_env()?;

    let mut executor = context.create_basic_executor();

    let node = executor.create_node("logger_node")?;
    let _subscription = node.clone().create_subscription(
    "camera_topic",
    move |_msg: RosImage| {
        let data = "Image has been recieved".to_string();

        // You can apply modifiers such as .once() to node.logger()
        // to dictate how the logging behaves.
        log!(
            node.logger(),
            "{data}",
        );
 
    }
)?;

// Any &str can be used as the logger name and have
// logging modifiers applied to it.
log_info!(
    "notice".once(),
    "Ready to begin logging sensor_msgs/msg/Image messages published to 'camera_topic'.",
);
log_warn!(
    "help",
    "Try running\n \
    $ ros2 run scout camera_topic",
);
    executor.spin(SpinOptions::default()).first_error()?;

    Ok(())
}

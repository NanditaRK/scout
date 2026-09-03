use rclrs::*;
use sensor_msgs::msg::Image as RosImage;
use std::error::Error;
use std::time::Duration;

use opencv::{
    core,
    prelude::*,
    videoio::VideoCapture,
    videoio::CAP_FFMPEG,
};

fn main() -> Result<(), Box<dyn Error>> {
    let context = Context::default_from_env()?;
    let mut executor = context.create_basic_executor();
    let node = executor.create_node("camera_node")?;
    let publisher = node.create_publisher::<RosImage>("camera_topic")?;
    let video_path = concat!(env!("CARGO_MANIFEST_DIR"), "/fish.mp4"); 
    println!("Video path: {}", video_path);
    println!(
    "File exists: {}",
    std::path::Path::new(video_path).exists()
    );

//    let mut camera = VideoCapture::new_def(0)?;
    let mut camera = VideoCapture::from_file(video_path, CAP_FFMPEG)?;
    if !camera.is_opened()? {
        return Err(format!("Could not open camera; {}", video_path).into());
    }

    log_info!(node.logger(), "Camera opened successfully.");

    let _timer = node.create_timer_repeating(
        Duration::from_millis(33),
        move || {
            let mut frame = core::Mat::default();

            // read one frame from the camera
            let ok = match camera.read(&mut frame) {
                Ok(ok) => ok,
                Err(e) => {
                    eprintln!("Camera read error: {}", e);
                    return;
                }
            };

            if !ok || frame.empty() {
                eprintln!("Got an empty frame");
                return;
            }

            // get width and height
            let width = frame.cols() as u32;
            let height = frame.rows() as u32;

            // get raw data bytes
            let data = match frame.data_bytes() {
                Ok(data) => data.to_vec(),
                Err(e) => {
                    eprintln!("Could not get frame data: {}", e);
                    return;
                }
            };

            let msg = RosImage {
                header: std_msgs::msg::Header::default(),
                height,
                width,
                encoding: "bgr8".to_string(),
                is_bigendian: 0,
                step: width * 3,
                data,
            };

            if let Err(e) = publisher.publish(msg) {
                eprintln!("Failed to publish image: {}", e);
            }
        },
    )?;

    log_info!(
        node.logger(),
        "Camera node started. Publishing images to 'camera_topic'."
    );

    executor
        .spin(SpinOptions::default())
        .first_error()?;

    Ok(())
}

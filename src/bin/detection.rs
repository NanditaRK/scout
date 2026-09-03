use fishsense_core::fish::fish_segmentation::FishSegmentation;
use ndarray::Array3;
use rclrs::*;
use sensor_msgs::msg::Image as RosImage;
use std::error::Error;
use std::sync::{Arc, Mutex};

fn ros_image_to_ndarray(msg: RosImage) -> Result<Array3<u8>, String> {
    if msg.encoding != "bgr8" {
        return Err(format!(
            "Expected bgr8 image, got {}",
            msg.encoding
        ));
    }

    let height = msg.height as usize;
    let width = msg.width as usize;

    let expected_len = height * width * 3;

    if msg.data.len() != expected_len {
        return Err(format!(
            "Expected {} bytes, got {}",
            expected_len,
            msg.data.len()
        ));
    }

    Array3::from_shape_vec(
        (height, width, 3),
        msg.data,
    )
    .map_err(|e| format!("Failed to create ndarray: {}", e))
}

fn main() -> Result<(), Box<dyn Error>> {
    let context = Context::default_from_env()?;
    let mut executor = context.create_basic_executor();
    let node = executor.create_node("detection_node")?;
    let logger = Logger::new("detection_logger")?;

    // load model
    let mut segmentation = FishSegmentation::new();
    segmentation.load_model()?;

    // callback needs mutable access to the model
    let segmentation = Arc::new(Mutex::new(segmentation));
    let segmentation_cb = Arc::clone(&segmentation);

    let _subscriber = node.create_subscription(
        "camera_topic",
        move |msg: RosImage| {
            let img = match ros_image_to_ndarray(msg) {
                Ok(img) => img,
                Err(e) => {
                    eprintln!("Image conversion error: {}", e);
                    return;
                }
            };

            let mut segmentation = segmentation_cb.lock().unwrap();

            match segmentation.inference_single(&img) {
                Ok(Some(_mask)) => {
                    log!(logger.info(), "Fish detected!");
                }
                Ok(None) => {
                    log!(logger.info(), "No fish detected!");
                }
                Err(e) => {
                    eprintln!("Fish segmentation error: {}", e);
                }
            }
        },
    )?;

    println!("Detection node started.");

    executor.spin(SpinOptions::default()).first_error()?;

    Ok(())
}

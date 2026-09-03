# Scout

ROS 2 package written in Rust for fish detection, segmentation, and length measurement using [UCSD E4E FishSense](https://github.com/UCSD-E4E/fishsense-core).

Scout uses the ROS 2 Rust client library (`rclrs`) to create the nodes.

## Requirements

Scout requires the following to run:
* **ROS 2 Humble**
* **Rust**
* **ROS 2 Rust (`rclrs`)** 

You should have ROS 2 Humble installed before setting up the ROS 2 Rust workspace.

---

# Installation

## 1. Install ROS 2 Humble

First, install ROS 2 Humble by following the official ROS 2 installation instructions:

https://docs.ros.org/en/humble/Installation.html

After installing ROS 2 Humble, source the ROS 2 environment:

```bash
source /opt/ros/humble/setup.bash # if you installed from binary
# or
source ~/ros2_humble/install/local_setup.bash # if you installed from source
```

You may want to add this command to your shell configuration (.bashrc etc) so that ROS 2 is sourced automatically when opening a new terminal.

---

## 2. Set up ROS 2 Rust

Scout uses `rclrs` and some other required libraries required when running ROS2 with Rust.

Follow the official `ros2_rust` installation and build instructions:

https://github.com/ros2-rust/ros2_rust

For ROS 2 Humble, follow the **ROS 2 Humble Hawksbill** section of the `ros2_rust` README.

The `ros2_rust` setup requires several dependencies and source repositories in the ROS 2 workspace. Follow their instructions to install these dependencies and build the ROS 2 Rust packages.

A typical workspace will look similar to:

```text
workspace/
└── src/
    ├── common_interfaces/
    ├── example_interfaces/
    ├── rcl_interfaces/
    ├── rosidl_core/
    ├── rosidl_defaults/
    ├── unique_identifier_msgs/
    ├── rosidl_rust/
    └── examples/
```

The exact contents of the workspace may vary depending on your `ros2_rust` setup.

After completing the `ros2_rust` setup, you should have a working ROS 2 Rust environment before continuing.

---

## 3. Clone Scout

Once the ROS 2 Rust workspace has been created, navigate to the `src` directory of the workspace.

For example:

```bash
cd ~/workspace/src
```

Clone Scout into the workspace:

```bash
git clone https://github.com/NanditaRK/scout.git scout
```

Your workspace should now look approximately like:

```text
fishsense_ws/
└── src/
    ├── common_interfaces/
    ├── example_interfaces/
    ├── rcl_interfaces/
    ├── rosidl_core/
    ├── rosidl_defaults/
    ├── unique_identifier_msgs/
    ├── rosidl_rust/
    ├── examples/
    └── scout/
        ├── Cargo.toml
        ├── Cargo.lock
        ├── package.xml
        └── src/
            ├── lib.rs
            └── bin/
                ├── camera.rs
                ├── detection.rs
                └── logger.rs
```

---

# FishSense Setup

Scout uses [UCSD E4E FishSense](https://github.com/UCSD-E4E/fishsense-core) for fish segmentation and other core utils.

The FishSense segmentation model is loaded at compile time, so the FishSense model must be available when building Scout.


# Building Scout

Navigate to the root of the ROS 2 workspace:

```bash
cd ~/workspace
```

Source ROS 2 Humble:

```bash
source /opt/ros/humble/setup.bash
# or
source ~/ros2_humble/install/local_setup.bash
```

Build the Scout package:

```bash
colcon build --packages-select scout
```

After the build completes, source the workspace:

```bash
source install/setup.bash
```

Scout should now be available as a ROS 2 package.

---

# Running Scout

Scout currently contains several ROS 2 nodes.

## Camera Node

The camera node reads frames from an OpenCV camera and publishes them as ROS 2 image messages.

Run:

```bash
ros2 run scout camera
```

The camera node publishes images on:

```text
/camera_topic
```

The message type is:

```text
sensor_msgs/msg/Image
```

Images are currently published using the:

```text
bgr8
```

encoding.

---

## Detection Node

The detection node subscribes to the camera topic and uses FishSense to determine whether a fish is present in the image.

Run:

```bash
ros2 run scout detection
```

The detection node subscribes to:

```text
/camera_topic
```

with message type:

```text
sensor_msgs/msg/Image
```

---

## Logger Node

The logger node can be run with:

```bash
ros2 run scout logger
```

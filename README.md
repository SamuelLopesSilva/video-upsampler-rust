# Weekend Project: Video Upsampler

## Overview

The "video-upsampler" project is a Rust-based application designed to upscale video resolution using some algorithms and techniques. The project utilizes the ffmpeg-next library for video processing, the image library for image manipulation, and the log, colored, env_logger libraries for logging.

**Key Features**

* Video upscaling using ffmpeg-next
* Image manipulation using the image library
* Currently implemented algorithms:
  * Bilinear Interpolation

My idea is to implement more algorithms over time, such as:

* Bicubic Interpolation
* Lanczos Resampling
* And maybe some AI stuff :)

## Run

To run the project on MacOS, Linux, and Windows, follow these steps:

**Prerequisites**

* Install Rust and Cargo on your system if you haven't already. You can download the installation package from the official Rust website.
* Ensure you have the necessary dependencies installed, including ffmpeg-next, image, log, colored, env_logger. These dependencies are specified in the `Cargo.toml` file and will be automatically installed when you run the project with Cargo.
* Install ffmpeg on your system. This is a required dependency for the project, and you can download it from the official ffmpeg website.

**Running the Project**

1. **Clone the Repository**
   Clone the "video-upsampler" repository to a local directory on your system using Git.
2. **Navigate to the Project Directory**
   Open a terminal or command prompt and navigate to the directory where you cloned the repository.
3. **Run the Project with Cargo**
   Use a command similar to this one to run the project:

   ```
   cargo run -- input.mp4 30 output.mp4
   ```

   In this example:

   * `input.mp4` is the input file to be upscaled.
   * `30` is the framerate of the input video.
   * `output.mp4` is the desired output file path. If the path for the output is not passed, the code will save the upscaled video in the same folder as the input file, with a filename that includes "_upscaled" inside a folder with the name of the input file name.

   The `Config` struct is initialized with these arguments, and the `main` function uses this configuration to extract frames from the input video, upscale them, and reassemble the video with the new frames and audio. Note: The `Config` struct expects (mandatory) the input file and framerate as command-line arguments. If these arguments are not provided, the program will exit with an error message.


## TODO

- [ ] Find some bilinear interpolation implementation to reduce bottleneck
- [ ] Verify if changing the frame extraction code makes sense (use OpenCV)

use std::env::Args;
use std::time::{Duration, Instant};

use image::{DynamicImage, GrayImage, Rgb};
use imageproc::drawing::draw_filled_rect_mut;
use imageproc::rect::Rect;

use rustface::{Detector, FaceInfo, ImageData};

// let mut output_file: &str = "test.png";

fn main() {
    let mut line = String::new();
    println!("Enter your name :");
    let read_line = std::io::stdin().read_line(&mut line).unwrap();
    let output_file = format!("{}.png", &line.trim());
    let output_file = output_file.as_str();
    println!("output file: {output_file}, filename bytes: {read_line}");
    let options = match Options::parse(std::env::args()) {
        Ok(options) => options,
        Err(message) => {
            println!("Failed to parse program arguments: {}", message);
            std::process::exit(1)
        }
    };

    let mut detector = match rustface::create_detector(options.model_path()) {
        Ok(detector) => detector,
        Err(error) => {
            println!("Failed to create detector: {}", error.to_string());
            std::process::exit(1)
        }
    };

    detector.set_min_face_size(20);
    detector.set_score_thresh(0.8); // first: 2.0, 0.95: found 22 faces
    detector.set_pyramid_scale_factor(0.85); // 0.8 is found 11 faces, 0.99 is found 14 faces but actually 13 faces
    detector.set_slide_window_step(2, 2);
    let image: DynamicImage = match image::open(options.image_path()) {
        Ok(image) => image,
        Err(message) => {
            println!("Failed to read image: {}", message);
            std::process::exit(1)
        }
    };

    let mut rgb = image.to_rgb8();
    let faces = detect_faces(&mut *detector, &image.to_luma8());

    for face in faces {
        let bbox = face.bbox();
        let rect = Rect::at(bbox.x(), bbox.y()).of_size(bbox.width(), bbox.height());

        draw_filled_rect_mut(&mut rgb, rect, Rgb([0, 0, 0]));
    }

    match rgb.save(output_file) {
        Ok(_) => println!("Saved result to {}", output_file),
        Err(message) => println!("Failed to save result to a file. Reason: {}", message),
    }
}

fn detect_faces(detector: &mut dyn Detector, gray: &GrayImage) -> Vec<FaceInfo> {
    let (width, height) = gray.dimensions();
    let mut image = ImageData::new(gray, width, height);
    let now = Instant::now();
    let faces = detector.detect(&mut image);
    println!(
        "Found {} faces in {} ms",
        faces.len(),
        get_millis(now.elapsed())
    );
    faces
}

fn get_millis(duration: Duration) -> u64 {
    duration.as_secs() * 1000u64 + u64::from(duration.subsec_nanos() / 1_000_000)
}

struct Options {
    image_path: String,
    model_path: String,
}

impl Options {
    fn parse(args: Args) -> Result<Self, String> {
        let args: Vec<String> = args.into_iter().collect();
        if args.len() != 3 {
            return Err(format!("Usage: {} <model-path> <image-path>", args[0]));
        }

        let model_path = args[1].clone();
        let image_path = args[2].clone();

        Ok(Options {
            image_path,
            model_path,
        })
    }

    fn image_path(&self) -> &str {
        &self.image_path[..]
    }

    fn model_path(&self) -> &str {
        &self.model_path[..]
    }
}

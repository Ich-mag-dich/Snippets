use image::{DynamicImage, GrayImage, Rgb};
use imageproc::drawing::draw_filled_rect_mut;
use imageproc::rect::Rect;
use rustface::{Detector, FaceInfo, ImageData, Rectangle};
use std::fs;
use std::io::{self, Write};
use std::path::{Path, PathBuf};
use std::time::{Duration, Instant};
fn main() {
    print!("Input image path: ");
    io::stdout().flush().unwrap();
    let mut input_path = String::new();
    io::stdin().read_line(&mut input_path).expect("input error");
    let input_path = input_path.trim();
    println!(
        "\n\n------------------------------------------\nimage path is: {}",
        input_path
    );
    // let files = PathBuf::from(input_path)
    //     .read_dir()
    //     .expect("Failed to read directory")
    //     .filter_map(|entry| {
    //         let path = &entry.unwrap();
    //         if path.file_type().unwrap().is_dir() {
    //             None
    //         } else {
    //             if path.path().file_name().unwrap_or_default() == ".DS_Store" {
    //                 None
    //             } else {
    //                 Some(path.path().to_str().unwrap().to_owned())
    //             }
    //         }
    //     })
    //     .collect::<Vec<String>>();
    let files: Vec<String> = Path::new(input_path)
        .read_dir()
        .expect("Failed to read directory")
        .filter_map(|entry| {
            let path = entry.ok()?.path();
            if path.is_dir() || path.file_name()?.to_str()? == ".DS_Store" {
                None
            } else {
                Some(path.to_str()?.to_owned())
            }
        })
        .collect::<Vec<String>>();
    let file_count = files.len();
    println!(
        "------------------------------------------\nimage count: {}",
        file_count
    );
    let mut detector: Box<dyn Detector> =
        match rustface::create_detector("./model/seeta_fd_frontal_v1.0.bin") {
            Ok(detector) => detector,
            Err(error) => {
                println!("Failed to create detector: {}", error.to_string());
                std::process::exit(1)
            }
        };
    detector.set_min_face_size(20);
    detector.set_max_face_size(500);
    detector.set_score_thresh(0.8); // first: 2.0, 0.95: found 22 faces
    detector.set_pyramid_scale_factor(0.9); // 0.8 is found 11 faces, 0.99 is found 14 faces but actually 13 faces
    detector.set_slide_window_step(4, 4);
    for (count, file) in files.iter().enumerate() {
        let file_path = file.replace("\"", "");
        println!("\n{}/{}\n{}", count + 1, file_count, file_path);
        let image: DynamicImage = image::open(&file_path)
            .expect(format!("Failed to read image: {}", &file_path).as_str());

        let mut rgb = image.to_rgb8();
        let faces = detect_faces(&mut *detector, &image.to_luma8());

        for face in faces {
            let bbox: &Rectangle = face.bbox();
            let rect: Rect = Rect::at(bbox.x(), bbox.y()).of_size(bbox.width(), bbox.height());

            draw_filled_rect_mut(&mut rgb, rect, Rgb([255, 255, 255]));
        }
        let output_file: PathBuf = PathBuf::from(format!(
            "./save/{}FD_{}",
            input_path.replace("img/", ""),
            file.as_str()
                .replace(&input_path.replace("img/", ""), "")
                .replace("img/", "")
        ));
        // println!("output file -> {}", output_file.display());
        match rgb.save(&output_file) {
            Ok(_) => println!("Saved result to {}", output_file.display()),
            Err(message) => {
                let create_dir_name: PathBuf =
                    PathBuf::from(format!("./save/{}", input_path.replace("img/", "")));
                fs::create_dir_all(&create_dir_name).expect("Failed to create directory");
                match rgb.save(&output_file) {
                    Ok(_) => println!("Saved result to {}", output_file.display()),
                    Err(message2) => println!("Can't save file, {}, {}", message, message2),
                }
            }
        }
    }
    println!("\n--------------------------------------------\nDone!");
}

fn detect_faces(detector: &mut dyn Detector, gray: &GrayImage) -> Vec<FaceInfo> {
    let (width, height): (u32, u32) = gray.dimensions();
    let mut image: ImageData = ImageData::new(gray, width, height);
    let now: Instant = Instant::now();
    let faces: Vec<FaceInfo> = detector.detect(&mut image);
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

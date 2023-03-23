use image::{DynamicImage, GrayImage, Rgb};
use imageproc::drawing::draw_filled_rect_mut;
use imageproc::rect::Rect;
use std::fs::read_dir;
use std::io::Write;
use std::time::{Duration, Instant};
use std::{fs, io};

use rustface::{Detector, FaceInfo, ImageData};

fn main() {
    print!("Input image path: ");
    io::stdout().flush().unwrap();
    let mut input_path = String::new();
    io::stdin().read_line(&mut input_path).expect("input error");
    let input_path = input_path.as_str().trim();
    println!(
        "\n\n------------------------------------------\nimage path is: {}",
        input_path
    );
    let path = read_dir(&input_path).unwrap();
    // println!("files: {}", &path.count());
    let mut file_count = 0;
    let mut files = vec![];
    let model_path = "./model/seeta_fd_frontal_v1.0.bin";
    for i in path {
        match i {
            Ok(file) => {
                if file.file_name() != ".DS_Store" {
                    let file_type_num = file.file_type();
                    let file_type_num = match file_type_num {
                        Ok(num) => num,
                        Err(_) => panic!(),
                    };

                    println!(
                        "------------------------------------------\n● {:?}",
                        file.file_name()
                    );
                    if file_type_num.is_dir() {
                        println!("this is not image");
                        continue;
                    }
                    file_count += 1;
                    files.push(file.file_name());
                }
            }

            Err(_) => println!("error"),
        }
    }

    println!("------------------------------------------\nimage count: {file_count}");
    let mut count = 0;
    while count < file_count {
        let mut detector = match rustface::create_detector(&model_path) {
            Ok(detector) => detector,
            Err(error) => {
                println!("Failed to create detector: {}", error.to_string());
                std::process::exit(1)
            }
        };
        let file_path = format!("{input_path}{:?}", &files[count]);

        let file_path = file_path.replace("\"", "");
        println!("\n{}/{file_count}\n{file_path}", &count + 1);
        detector.set_min_face_size(20);
        detector.set_max_face_size(400);
        detector.set_score_thresh(0.9); // first: 2.0, 0.95: found 22 faces
        detector.set_pyramid_scale_factor(0.9); // 0.8 is found 11 faces, 0.99 is found 14 faces but actually 13 faces
        detector.set_slide_window_step(4, 4);
        let image: DynamicImage = match image::open(&file_path) {
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

            draw_filled_rect_mut(&mut rgb, rect, Rgb([255, 255, 255]));
        }
        let output_file = format!(
            "./save/{input_path}save_{}",
            &files[count].to_str().unwrap()
        );
        match rgb.save(&output_file) {
            Ok(_) => println!("Saved result to {}", output_file),
            Err(message) => {
                let create_dir_name = format!("./save/{input_path}");
                fs::create_dir_all(create_dir_name.as_str()).unwrap();
                match rgb.save(&output_file) {
                    Ok(_) => println!("Save result to {}", output_file),
                    Err(message2) => println!("can't save file, {}, {}", message, message2),
                }
            }
        }
        count += 1;
    }
    println!("\n--------------------------------------------\nDone!");
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

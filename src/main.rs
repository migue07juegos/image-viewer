use arboard::{Clipboard, ImageData};
use image::imageops::{rotate270, rotate90};
use rfd::FileDialog;
use std::borrow::Cow;
use std::{
    env,
    path::PathBuf,
    sync::{Arc, Mutex},
};
slint::include_modules!();

fn main() -> Result<(), slint::PlatformError> {
    let args: Vec<String> = env::args().collect();
    let image_path: PathBuf = (&args[1]).into();

    let mut clipboard = Clipboard::new().unwrap();

    let source_image = image::open(&image_path)
        .expect("Error loading image")
        .into_rgba8();
    let arc_image = Arc::new(Mutex::new(source_image.clone()));

    let ui = MainWindow::new()?;
    let ui_weak = ui.as_weak();

    ui.set_image_data(slint::Image::from_rgba8(
        slint::SharedPixelBuffer::clone_from_slice(
            source_image.as_raw(),
            source_image.width(),
            source_image.height(),
        ),
    ));
    {
        let copied_image = arc_image.clone();
        let ui_weak = ui_weak.clone();

        ui.on_rotate_clockwise(move || {
            let copied_image = copied_image.clone();
            let _ = ui_weak.upgrade_in_event_loop(move |ui| {
                let mut rotated_image = copied_image.lock().unwrap();
                *rotated_image = rotate90(&*rotated_image);
                ui.set_image_data(slint::Image::from_rgba8(
                    slint::SharedPixelBuffer::clone_from_slice(
                        rotated_image.as_raw(),
                        rotated_image.width(),
                        rotated_image.height(),
                    ),
                ))
            });
        });
    }

    {
        let copied_image = arc_image.clone();
        let ui_weak = ui_weak.clone();

        ui.on_rotate_anti_clockwise(move || {
            let copied_image = copied_image.clone();
            let _ = ui_weak.upgrade_in_event_loop(move |ui| {
                let mut rotated_image = copied_image.lock().unwrap();
                *rotated_image = rotate270(&*rotated_image);
                ui.set_image_data(slint::Image::from_rgba8(
                    slint::SharedPixelBuffer::clone_from_slice(
                        rotated_image.as_raw(),
                        rotated_image.width(),
                        rotated_image.height(),
                    ),
                ))
            });
        });
    }

    {
        let copied_image = arc_image.clone();
        let ui_weak = ui_weak.clone();
        let source_image = source_image.clone();

        ui.on_rotate_default(move || {
            let source_image = source_image.clone();
            let copied_image = copied_image.clone();
            let _ = ui_weak.upgrade_in_event_loop(move |ui| {
                let mut rotated_image = copied_image.lock().unwrap();
                *rotated_image = source_image;
                ui.set_image_data(slint::Image::from_rgba8(
                    slint::SharedPixelBuffer::clone_from_slice(
                        rotated_image.as_raw(),
                        rotated_image.width(),
                        rotated_image.height(),
                    ),
                ))
            });
        });
    }

    {
        let copied_image = arc_image.clone();

        ui.on_copy(move || {
            let copied_image = copied_image.lock().unwrap();
            let _ = clipboard.set_image(ImageData {
                width: copied_image.width().try_into().unwrap(),
                height: copied_image.height().try_into().unwrap(),
                bytes: Cow::from(copied_image.as_raw()),
            });
        });
    }

    {
        let copied_image = arc_image.clone();
        let image_path = image_path.clone();

        ui.on_save(move || {
            let files_dialog = FileDialog::new()
                .add_filter("Image", &["png"])
                .set_directory(image_path.parent().unwrap())
                .set_file_name(image_path.file_stem().unwrap().to_string_lossy())
                .save_file();
            let copied_image = copied_image.lock().unwrap();

            match files_dialog {
                Some(path) => {
                    if cfg!(target_os="linux") {
                        println!("{}", format!("{}.png", path.to_string_lossy()));
                        copied_image
                            .save(format!("{}.png", path.to_string_lossy()))
                            .expect("Error saving");
                    } else {
                        copied_image.save(path).expect("Error saving")
                    }
                }
                None => (),
            }
        });
    }

    ui.set_image_data(slint::Image::from_rgba8(
                    slint::SharedPixelBuffer::clone_from_slice(
                        source_image.as_raw(),
                        source_image.width(),
                        source_image.height(),
                    ),
                ));

    ui.run()
}

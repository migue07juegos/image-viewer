use std::{
    env,
    path::PathBuf,
    sync::{Arc, Mutex},
};

use image::imageops::{rotate270, rotate90};
slint::include_modules!();

fn main() -> Result<(), slint::PlatformError> {
    let args: Vec<String> = env::args().collect();
    let image_path: PathBuf = (&args[1]).into();

    let source_image = image::open(&image_path)
        .expect("Error loading image")
        .into_rgba8();
    let cell_image = Arc::new(Mutex::new(source_image.clone()));

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
        let copied_image = cell_image.clone();
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
        let copied_image = cell_image.clone();
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
        let copied_image = cell_image.clone();
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

    ui.set_image_data(slint::Image::load_from_path(&image_path).unwrap());

    ui.run()
}

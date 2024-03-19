use std::{env, path::PathBuf};
slint::include_modules!();

fn main() -> Result<(), slint::PlatformError> {
    let args: Vec<String> = env::args().collect();
    let image_path: PathBuf = (&args[1]).into();
    
    // let source_image = image::open(&image_path).expect("Error loading image").into_rgba8();

    let ui = MainWindow::new()?;
    // ui.set_original_image(slint::Image::from_rgba8(
    //     slint::SharedPixelBuffer::clone_from_slice(
    //         source_image.as_raw(),
    //         source_image.width(),
    //         source_image.height(),
    //     ),
    // ));
    ui.set_original_image(slint::Image::load_from_path(&image_path).unwrap());

    ui.run()
}

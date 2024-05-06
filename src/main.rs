slint::include_modules!();

fn main() -> Result<(), slint::PlatformError> {
    use slint::Model;

    let ui = AppWindow::new()?;
    ui.run()
}
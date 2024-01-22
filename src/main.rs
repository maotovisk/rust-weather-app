use slint::ModelRc;

slint::include_modules!();
struct CityInfo {
    city: String,
    temperature: i32,
}
fn main() -> Result<(), slint::PlatformError> {
    let ui = FodaseWindow::new()?;

    ui.global::<ApiManagement>().on_search_city(|city_name| {
        println!("Searching for city: {}", city_name);

        
    });

    ui.run()
}

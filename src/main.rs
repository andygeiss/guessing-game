use rand::Rng;
use std::cmp::Ordering;

slint::include_modules!();

fn main() -> Result<(), slint::PlatformError> {
    
    let ui = AppWindow::new()?;
    let ui_handle = ui.as_weak();

    ui.on_compare_with_secret_number(move || {

        let ui = ui_handle.unwrap();

        let guess: u32 = match ui.get_guess().parse() {
            Ok(num) => num,
            Err(_) => {
                ui.set_message("Please enter a number!".into());
                return;
            }
        };

        let secret_number: u32 = rand::thread_rng().gen_range(1..100);

        match guess.cmp(&secret_number) {
            Ordering::Less => ui.set_message("Too small!".into()),
            Ordering::Greater => ui.set_message("Too big!".into()),
            Ordering::Equal => ui.set_message("You win!".into()),
        }
        
    });

    ui.run()
}

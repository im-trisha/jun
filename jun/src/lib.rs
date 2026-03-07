// TODO: CURRENTLY DOESN'T WORK ON WEB BECAUSE OF FILE PICKER, I THINK, AND 100% BECAUSE I READ USING IO FNS
// TODO: ON RELEASE CONVERT UNWRAPS TO ERROR HANDLING

mod app;
mod i18n;
mod views;

pub use app::JunApp;
pub use i18n::Language;

#[macro_export]
macro_rules! try_i18n {
    ($app:expr, $expr:expr, $msg:expr) => {
        match $expr {
            Ok(v) => v,
            Err(_e) => {
                $app.errors.push($msg.into());
                return;
            }
        }
    };
}

use tauri::{
  plugin::{Builder, TauriPlugin},
  Manager, Runtime,
};

pub use models::*;

#[cfg(desktop)]
mod desktop;
#[cfg(mobile)]
mod mobile;

mod commands;
mod error;
mod models;

pub use error::{Error, Result};

#[cfg(desktop)]
use desktop::MobileGoogleAuth;
#[cfg(mobile)]
use mobile::MobileGoogleAuth;

/// Extensions to [`tauri::App`], [`tauri::AppHandle`] and [`tauri::Window`] to access the mobile-google-auth APIs.
pub trait MobileGoogleAuthExt<R: Runtime> {
  fn mobile_google_auth(&self) -> &MobileGoogleAuth<R>;
}

impl<R: Runtime, T: Manager<R>> crate::MobileGoogleAuthExt<R> for T {
  fn mobile_google_auth(&self) -> &MobileGoogleAuth<R> {
    self.state::<MobileGoogleAuth<R>>().inner()
  }
}

/// Initializes the plugin.
pub fn init<R: Runtime>() -> TauriPlugin<R> {
  Builder::new("mobile-google-auth")
    .invoke_handler(tauri::generate_handler![commands::ping])
    .setup(|app, api| {
      #[cfg(mobile)]
      let mobile_google_auth = mobile::init(app, api)?;
      #[cfg(desktop)]
      let mobile_google_auth = desktop::init(app, api)?;
      app.manage(mobile_google_auth);
      Ok(())
    })
    .build()
}

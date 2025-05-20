use serde::de::DeserializeOwned;
use tauri::{plugin::PluginApi, AppHandle, Runtime};

use crate::models::*;

pub fn init<R: Runtime, C: DeserializeOwned>(
  app: &AppHandle<R>,
  _api: PluginApi<R, C>,
) -> crate::Result<MobileGoogleAuth<R>> {
  Ok(MobileGoogleAuth(app.clone()))
}

/// Access to the mobile-google-auth APIs.
pub struct MobileGoogleAuth<R: Runtime>(AppHandle<R>);

impl<R: Runtime> MobileGoogleAuth<R> {
  pub fn ping(&self, payload: PingRequest) -> crate::Result<PingResponse> {
    Ok(PingResponse {
      value: payload.value,
    })
  }
}

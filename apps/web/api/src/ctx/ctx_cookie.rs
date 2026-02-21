use axum::extract::FromRef;
use axum_extra::extract::cookie::Key;

use crate::ctx::AppContext;

/// Impl PrivateCookieJar
impl FromRef<AppContext> for Key {
    fn from_ref(state: &AppContext) -> Self {
        (*state.jkey).clone()
    }
}

use serde::{Deserialize, Serialize};

/// User data session
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct AppUser {
    pub id: i32,
    pub email: String,
}

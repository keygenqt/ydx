use axum::extract::{FromRequest, Request};
use axum_extra::extract::Multipart;

use crate::ctx::AppContext;
use crate::{
    api::api_response::ApiResponse,
    domain::image::model::{CardImage, ImageModel, SetIcon, UserIcon},
};
use axum::extract::FromRef;

// Max size file image - 10Mb
const MAX_SIZE: usize = 10 * 1024 * 1024;

macro_rules! define_image_extractor {
    ($name:ident, $limit:ty) => {
        pub struct $name(pub ImageModel);

        impl<S> FromRequest<S> for $name
        where
            S: Send + Sync,
            AppContext: FromRef<S>,
        {
            type Rejection = ApiResponse;

            async fn from_request(req: Request, state: &S) -> Result<Self, Self::Rejection> {
                // get app context for config
                let app_context = AppContext::from_ref(state);
                // get data list
                let mut multipart = Multipart::from_request(req, state)
                    .await
                    .map_err(|e| ApiResponse::new_400().with_message(&e.to_string()))?;
                // loop data
                while let Some(field) = multipart
                    .next_field()
                    .await
                    .map_err(|e| ApiResponse::new_400().with_message(&e.to_string()))?
                {
                    // check content type
                    let content_type = field
                        .content_type()
                        .map(|ct| ct.to_string())
                        .unwrap_or_else(|| "application/octet-stream".to_string());
                    // check mime type
                    if !content_type.starts_with("image/") {
                        return Err(
                            ApiResponse::new_400().with_message(&format!("File is not an image"))
                        );
                    }
                    // read file
                    let data = match field.bytes().await {
                        Ok(bytes) => bytes.to_vec(),
                        Err(e) => {
                            return Err(ApiResponse::new_400()
                                .with_message(&format!("Failed to read file: {}", e)));
                        }
                    };
                    // check size file
                    if data.len() > MAX_SIZE {
                        return Err(ApiResponse::new_400().with_message(&format!(
                            "File too large (max {}MB)",
                            MAX_SIZE / 1024 / 1024
                        )));
                    }
                    // create image model
                    let model = ImageModel::new::<$limit>(&app_context.conf, &data).await?;
                    // result
                    return Ok($name(model));
                }

                Err(ApiResponse::new_400().with_message("No file uploaded"))
            }
        }
    };
}

define_image_extractor!(ApiUserIcon, UserIcon);
define_image_extractor!(ApiSetIcon, SetIcon);
define_image_extractor!(ApiCardImage, CardImage);

use crate::{
    async_trait,
    extract::Extractor,
    header::{HeaderValue, CONTENT_TYPE},
    responder::Responder,
    Method, Request, Response, StatusCode,
};

use http_kit::{Error, ResultExt};
use serde::{de::DeserializeOwned, Serialize};
use serde_urlencoded::{from_str, to_string};

/// Extract form from request body.
#[derive(Debug)]
pub struct Form<T>(pub T);

const APPLICATION_WWW_FORM_URLENCODED: HeaderValue =
    HeaderValue::from_static("application/x-www-form-urlencoded");

impl<T: Serialize> Responder for Form<T> {
    fn respond_to(self, _request: &Request, response: &mut Response) -> crate::Result<()> {
        response.replace_body(to_string(&self.0)?);
        response.insert_header(CONTENT_TYPE, APPLICATION_WWW_FORM_URLENCODED);
        Ok(())
    }
}

impl_error!(
    FormContentTypeError,
    "Expected content type `application/x-www-form-urlencoded`",
    "This error occurs for a dismatched content type."
);

#[async_trait]
impl<T: DeserializeOwned> Extractor for Form<T> {
    async fn extract(request: &mut Request) -> crate::Result<Self> {
        if request
            .get_header(CONTENT_TYPE)
            .ok_or(FormContentTypeError)?
            != APPLICATION_WWW_FORM_URLENCODED
        {
            return Err(FormContentTypeError).status(StatusCode::UNSUPPORTED_MEDIA_TYPE);
        }

        if request.method() == Method::GET {
            let data = request.uri().query().unwrap_or_default();
            extract(data)
        } else {
            let data = request.take_body()?.into_string().await?;
            extract(&data)
        }
    }
}

fn extract<T: DeserializeOwned>(data: &str) -> Result<Form<T>, Error> {
    Ok(Form(from_str(data).map_err(|_| {
        Error::new(FormContentTypeError, StatusCode::UNSUPPORTED_MEDIA_TYPE)
    })?))
}

impl_deref!(Form);

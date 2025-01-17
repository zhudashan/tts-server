use std::pin::Pin;
use std::task::{Context, Poll};
use actix_web::body::{BoxBody, MessageBody};
use actix_web::http::{header, StatusCode};
use actix_web::http::header::HeaderValue;
use actix_web::HttpResponse;
use bytes::Bytes;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, PartialEq, Debug)]
pub struct ApiBaseResponse<T>
    where
        T: Serialize,
{
    pub code: i32,
    // #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<T>,
    pub msg: String,
}

impl<T> ToString for ApiBaseResponse<T>
    where
        T: Serialize,
{
    fn to_string(&self) -> String {
        serde_json::to_string(&self).unwrap()
    }
}

#[allow(dead_code)]
impl<T> ApiBaseResponse<T>
    where
        T: Serialize,
{
    #[allow(dead_code)]
    pub fn success(data: Option<T>) -> ApiBaseResponse<T> {
        ApiBaseResponse {
            code: 200,
            data: Option::<T>::from(data),
            msg: "success".to_owned(),
        }
    }

    #[allow(dead_code)]
    pub fn error<Y:Into<String>>(msg: Y) -> ApiBaseResponse<T> {
        let df = ApiBaseResponse {
            code: 500,
            data: Option::<T>::from(None),
            msg: msg.into(),
        };
        df
    }

    #[allow(dead_code)]
    pub fn error_by_status_code<Y:Into<String>>(code: i32, msg: Y) -> ApiBaseResponse<T> {
        ApiBaseResponse {
            code,
            data: Option::<T>::from(None),
            msg: msg.into(),
        }
    }


}

#[allow(dead_code)]
pub fn success_as_value(data: serde_json::Value) -> ApiBaseResponse<serde_json::Value> {
    ApiBaseResponse {
        code: 200,
        data: Some(data),
        msg: "success".to_owned(),
    }
}

impl<T> MessageBody for ApiBaseResponse<T>
    where
        T: Serialize,
{
    type Error = std::convert::Infallible;

    fn size(&self) -> actix_web::body::BodySize {
        let payload_string = ApiBaseResponse::to_string(&self);
        let payload_bytes = Bytes::from(payload_string);
        actix_web::body::BodySize::Sized(payload_bytes.len() as u64)
    }

    fn poll_next(
        self: Pin<&mut Self>,
        _cx: &mut Context<'_>,
    ) -> Poll<Option<Result<Bytes, Self::Error>>> {
        let payload_string = ApiBaseResponse::to_string(&self);
        let payload_bytes = Bytes::from(payload_string);
        Poll::Ready(Some(Ok(payload_bytes)))
    }
}

impl<T> Into<Bytes> for ApiBaseResponse<T>
    where
        T: Serialize,
{
    fn into(self) -> Bytes {
        let payload_string = Self::to_string(&self);
        let payload_bytes = Bytes::from(payload_string);
        payload_bytes
    }
}

impl<T> Into<HttpResponse<BoxBody>> for ApiBaseResponse<T>
    where
        T: Serialize,
{
    fn into(self) -> HttpResponse<BoxBody> {
        let bytes: Bytes = self.into();
        let mut res = HttpResponse::with_body(StatusCode::OK, bytes);
        res.headers_mut().insert(
            header::CONTENT_TYPE,
            HeaderValue::from_static("application/json; charset=utf-8"),
        );
        res.map_into_boxed_body()
    }
}

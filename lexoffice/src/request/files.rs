use crate::error::Error;
use crate::model::File;
use crate::request::Endpoint;
use crate::request::Request;
use crate::result::Result;
use crate::util::error_for_lexoffice;
use crate::util::to_json_response;
use reqwest::multipart::{Form, Part};
use reqwest::Method;
use reqwest::Response;
use reqwest::Url;
use serde::Deserialize;
use std::borrow::Cow;
use uuid::Uuid;

impl Endpoint for Request<File> {
    const ENDPOINT: &'static str = "files";
}

#[derive(Deserialize, Debug)]
struct FileResponse {
    pub id: Uuid,
}

impl Request<File> {
    /// This method creates an `Url` that is used to address the object
    /// identified by `uuid`.
    pub fn by_id_url<I>(&self, uuid: I) -> Result<Url>
    where
        I: Into<Uuid> + Send + Sync,
    {
        let uuid: Uuid = uuid.into();
        let mut url = self.url();
        url.path_segments_mut()
            .map_err(|_| Error::UrlCannotBeBase)?
            .push(&uuid.to_string());
        Ok(url)
    }

    /// This method requests an object identified by `uuid`.
    pub async fn by_id_str(self, uuid: &str) -> Result<Response> {
        self.by_id(Uuid::parse_str(uuid)?).await
    }

    /// This method requests an object identified by `uuid`.
    pub async fn by_id<I>(self, uuid: I) -> Result<Response>
    where
        I: Into<Uuid> + Send + Sync,
    {
        let uuid: Uuid = uuid.into();
        let url = self.by_id_url(uuid)?;
        error_for_lexoffice(
            self.client.http_builder(Method::GET, url).send().await?,
        )
        .await
    }

    /// Uploads a file to lexoffice
    pub async fn upload<P>(self, file_part: P) -> Result<Uuid>
    where
        P: Into<Part> + Send + Sync,
    {
        let file_part = file_part.into();
        let url = self.url();
        let form = Form::new().part("file", file_part).text("type", "voucher");
        to_json_response::<FileResponse>(
            self.client()
                .http_builder(Method::POST, url)
                .multipart(form),
        )
        .await
        .map(|x| x.id)
    }

    /// Uploads arbitrary data as file to lexoffice
    pub async fn upload_bytes<B>(
        self,
        mime: &'static mime::Mime,
        bytes: B,
    ) -> Result<Uuid>
    where
        B: Into<Cow<'static, [u8]>>,
    {
        let file_part = Part::bytes(bytes).mime_str(mime.as_ref())?;
        self.upload(file_part).await
    }

    /// Uploads a file from a path to lexoffice
    #[cfg(feature = "fs")]
    pub async fn upload_path<P>(self, path: P) -> Result<Uuid>
    where
        P: AsRef<std::path::Path> + Send + Sync,
    {
        use crate::mime::*;
        use reqwest::Body;

        let path = path.as_ref();
        let file = tokio::fs::File::open(path).await?;
        let stream = crate::fs::BytesStream::new(file);
        let mime = path.mime();
        let part = Part::stream(Body::wrap_stream(stream))
            .mime_str(mime.as_ref())?
            .file_name(format!("document.{}", mime.extension()));
        self.upload(part).await
    }
}

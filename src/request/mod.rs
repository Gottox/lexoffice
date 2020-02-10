mod contacts;
mod credit_notes;
mod event_subscriptions;
mod invoices;
mod order_confirmations;
mod profile;
pub mod voucher_list;
mod quotations;

use crate::client::RequestBuilder;
use crate::model::server_resource::ServerResource;
use crate::model::Page;
use async_trait::async_trait;
use serde::de::DeserializeOwned;
use std::error::Error;
use std::marker::PhantomData;
use std::str::FromStr;
use std::sync::Arc;
use reqwest::Url;
use uuid::Uuid;

#[derive(Clone, Debug)]
pub struct Request<T> {
    builder: Arc<RequestBuilder>,
    url: Url,
    phantom: PhantomData<T>,
}

impl<T> Request<T> {
    pub fn new(builder: Arc<RequestBuilder>) -> Self {
        let url = builder.base_url.clone();
        Request {
            builder,
            url,
            phantom: PhantomData,
        }
    }
}

impl<T> Requestable for Request<T>
where
    Self: Endpoint,
{
    fn url(&self) -> Url {
        let mut url = self.url.clone();
        url.path_segments_mut().unwrap().push(Self::ENDPOINT);
        url
    }
    fn builder(&self) -> &RequestBuilder {
        &self.builder
    }
}

pub trait Endpoint {
    const ENDPOINT: &'static str;
}

pub trait Requestable
where
    Self: Sized,
{
    fn url(&self) -> Url;
    fn builder(&self) -> &RequestBuilder;
}

#[async_trait]
pub trait Storable<T>
where
    Self: Requestable,
{
    async fn save(self) -> Result<ServerResource<T>, Box<dyn Error>>;
}

#[async_trait]
pub trait Simple<T>
where
    Self: Requestable,
    T: DeserializeOwned,
{
    async fn get(self) -> Result<ServerResource<T>, Box<dyn Error>>
    where
        T: 'async_trait,
    {
        let url = self.url();
        let builder = self.builder();
        Ok(builder.json(&url).await?)
    }
}

#[async_trait]
pub trait Paginated<T>
where
    Self: Requestable + Sized,
    T: DeserializeOwned,
{
    async fn page_size(
        self,
        page: usize,
        size: usize,
    ) -> Result<Page<T>, Box<dyn Error>>
    where
        T: 'async_trait,
    {
        let mut url = self.url();
        url.query_pairs_mut()
            .append_pair("page", &page.to_string())
            .append_pair("size", &size.to_string());

        let builder = self.builder();
        Ok(builder.json(&url).await?)
    }

    async fn page(self, page: usize) -> Result<Page<T>, Box<dyn Error>>
    where
        T: 'async_trait,
    {
        let mut url = self.url();
        url.query_pairs_mut().append_pair("page", &page.to_string());

        let builder = self.builder();
        Ok(builder.json(&url).await?)
    }
}

#[async_trait]
pub trait ById<T>
where
    T: DeserializeOwned,
    Self: Sized + Requestable,
{
    fn by_id_url<I>(&self, uuid: I) -> Result<Url, Box<dyn Error>>
    where
        I: Into<Uuid> + Send + Sync,
    {
        let uuid: Uuid = uuid.into();
        let mut url = self.url();
        url.path_segments_mut()
            .map_err(|_| "cannot be base")?
            .push(&uuid.to_string());
        Ok(url)
    }

    async fn by_id_str(
        self,
        uuid: &str,
    ) -> Result<ServerResource<T>, Box<dyn Error>>
    where
        T: 'async_trait,
    {
        let builder = self.builder();
        let url = self.by_id_url(Uuid::from_str(uuid)?)?;
        Ok(builder.json(&url).await?)
    }
    async fn by_id<I>(
        self,
        uuid: I,
    ) -> Result<T, Box<dyn Error>>
    where
        T: 'async_trait,
        I: Into<Uuid> + Send + Sync,
    {
        let builder = self.builder();
        let url = self.by_id_url(uuid)?;
        Ok(builder.json(&url).await?)
    }
}
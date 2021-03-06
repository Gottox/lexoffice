use lexoffice::request::RequestWithState;
use lexoffice::request::{ById, Deletable, Endpoint, HasId};
use lexoffice::Result;
use serde::de::DeserializeOwned;
use serde::ser::Serialize;
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
pub struct DeletableOpt {
    /// uuid of the element
    id: String,
}

impl DeletableOpt {
    pub async fn exec<T, U>(
        &self,
        request: RequestWithState<T, U>,
    ) -> Result<()>
    where
        RequestWithState<T, U>: Deletable + ById + Endpoint + Clone,
        T: DeserializeOwned + Clone + Serialize + HasId,
        U: Clone,
    {
        request.delete_str(&self.id).await
    }
}

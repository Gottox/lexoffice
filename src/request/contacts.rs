use crate::model::Contact;
use crate::request::impls::ById;
use crate::request::impls::Paginated;
use crate::request::Endpoint;
use crate::request::Request;

pub type ContactRequest = Request<Contact, ()>;

impl Endpoint for ContactRequest {
    const ENDPOINT: &'static str = "contacts";
}

/// # Examples
///
/// ``` no_run
/// use lexoffice::client::{ApiKey, Client};
/// use lexoffice::model::Contact;
///
/// # use lexoffice::Result;
/// # #[tokio::main]
/// # async fn main() -> Result<()> {
/// let client = Client::new(ApiKey::try_default().await?);
/// let uuid = uuid::Uuid::parse_str("f4add52b-44e3-474a-b718-890885094d9a")?;
/// let contacts = client.request::<Contact>().by_id(uuid).await?;
/// println!("{:#?}", contacts);
/// # Ok(())
/// # }
/// ```
///
impl ById for ContactRequest {}

/// # Examples
///
/// ``` no_run
/// use lexoffice::client::{ApiKey, Client};
/// use lexoffice::model::Contact;
///
/// # use lexoffice::Result;
/// # #[tokio::main]
/// # async fn main() -> Result<()> {
/// let client = Client::new(ApiKey::try_default().await?);
/// let contacts = client.request::<Contact>().page(0).await?;
/// println!("{:#?}", contacts);
/// # Ok(())
/// # }
/// ```
///
impl Paginated for ContactRequest {}

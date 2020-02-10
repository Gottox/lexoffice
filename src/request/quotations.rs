use crate::model::Quotation;
use crate::request::ById;
use crate::request::Paginated;
use crate::request::Request;
use crate::request::Endpoint;

/// # Examples
///
/// ``` no_run
/// use lexoffice::client::{ Client, ApiKey };
/// use lexoffice::model::Quotation;
/// use lexoffice::request::ById;
/// 
/// # use std::error::Error;
/// # #[tokio::main]
/// # async fn main() -> Result<(), Box<dyn Error>> {
/// let client = Client::new(ApiKey::try_default().await?);
/// let uuid = uuid::Uuid::parse_str("f4add52b-44e3-474a-b718-890885094d9a")?;
/// let quotations = client.request::<Quotation>().by_id(uuid).await?;
/// println!("{:#?}", quotations);
/// # Ok(())
/// # }
/// ```
///
impl ById<Quotation> for Request<Quotation> {
}

/// # Examples
///
/// ``` no_run
/// use lexoffice::client::{ Client, ApiKey };
/// use lexoffice::model::Quotation;
/// use lexoffice::request::Paginated;
/// 
/// # use std::error::Error;
/// # #[tokio::main]
/// # async fn main() -> Result<(), Box<dyn Error>> {
/// let client = Client::new(ApiKey::try_default().await?);
/// let quotations = client.request::<Quotation>().page(0).await?;
/// println!("{:#?}", quotations);
/// # Ok(())
/// # }
/// ```
///
impl Paginated<Quotation> for Request<Quotation> {
}

impl Endpoint for Request<Quotation> {
    const ENDPOINT: &'static str = "quotations";
}
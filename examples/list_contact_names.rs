use lexoffice::client::{ApiKey, Client};
use lexoffice::model::Contact;
use lexoffice::request::Paginated;
use tokio::stream::StreamExt;

use std::error::Error;
#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let client = Client::new(ApiKey::try_default().await?);
    let mut contacts = client.request::<Contact>().stream();

    while let Some(contact) = contacts.next().await {
        let contact = contact?;
        println!("{}", contact.company.name);
    }
    Ok(())
}

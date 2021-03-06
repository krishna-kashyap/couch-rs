use couch_rs::document::TypedCouchDocument;
use couch_rs::types::document::DocumentId;
use couch_rs::CouchDocument;
use reqwest::StatusCode;
use serde::{Deserialize, Serialize};

const TEST_DB: &str = "test_db";

#[derive(Serialize, Deserialize, CouchDocument, Default, Debug)]
pub struct TestDoc {
    /// _ids are are the only unique enforced value within CouchDB so you might as well make use of this.
    /// CouchDB stores its documents in a B+ tree. Each additional or updated document is stored as
    /// a leaf node, and may require re-writing intermediary and parent nodes. You may be able to take
    /// advantage of sequencing your own ids more effectively than the automatically generated ids if
    /// you can arrange them to be sequential yourself. (https://docs.couchdb.org/en/stable/best-practices/documents.html)
    #[serde(skip_serializing_if = "String::is_empty")]
    pub _id: DocumentId,
    /// Document Revision, provided by CouchDB, helps negotiating conflicts
    #[serde(skip_serializing_if = "String::is_empty")]
    pub _rev: String,
    pub first_name: String,
    pub last_name: String,
}

#[tokio::main]
async fn main() {
    println!("Connecting...");

    // Prepare the Sofa client
    let client = couch_rs::Client::new_local_test().unwrap();

    // This command gets a reference to an existing database, or it creates a new one when it does
    // not yet exist.
    let db = client.db(TEST_DB).await.unwrap();

    let td = TestDoc {
        _id: "1234".to_string(),
        _rev: "".to_string(),
        first_name: "John".to_string(),
        last_name: "Doe".to_string(),
    };

    // check if the document already exists
    match db.get::<TestDoc>(&td._id).await {
        Ok(e) => {
            println!("Document has been previously created with Rev: {}", e._rev);
            println!("Name: {} {}", e.first_name, e.last_name);
        }
        Err(e) => {
            match e.status {
                StatusCode::NOT_FOUND => {
                    // create the document
                    match db.create(serde_json::to_value(td).unwrap()).await {
                        Ok(r) => println!("Document was created with ID: {} and Rev: {}", r.get_id(), r.get_rev()),
                        Err(err) => println!("Oops: {:?}", err),
                    }
                }
                _ => {
                    println!("Unexpected error: {:?}", e);
                }
            }
        }
    }

    println!("All operations are done")
}

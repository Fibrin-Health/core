
mod erh_indexer;


use candid::{candid_method, export_service};
use ic_cdk::api::call::ManualReply;
use std::cell::RefCell;


// Upddate comments to natspec


thread_local! {
    static EHR_INDEXER: RefCell<erh_indexer::EHRIndexer> = RefCell::new(erh_indexer::EHRIndexer::new());
}


#[ic_cdk::query]
fn greet(name: String) -> String {
    format!("Hello, {}!", name)
}

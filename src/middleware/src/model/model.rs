//  Not for production use

use crate::{ctx::{self, Ctx}, Error, Result};
use serde::{Deserialize, Serialize};
use std::sync::{Arc, Mutex};

#[derive(Debug, Clone, Serialize)]
pub struct Ehr {
    pub id: u64,
    pub cid: u64,
    pub title: String,
} 

#[derive(Deserialize)]
pub struct EhrForCreate {
    pub title: String,
}

// Model controller
#[derive(Clone)]
pub struct ModelController {
    ehr_store: Arc<Mutex<Vec<Option<Ehr>>>>,
}


// Constructor
impl ModelController {
    pub async fn new() -> Result<Self> {
        Ok(Self {
            ehr_store: Arc::default()
        })
    }
}

impl ModelController {
    pub async fn create_ehr(&self, ctx: Ctx, ehr_fc: EhrForCreate) -> Result<Ehr> {
        let mut store = self.ehr_store.lock().unwrap();
        let id = store.len() as u64;
        let ehr = Ehr {
            id,
            cid: ctx.user_id(),
            title: ehr_fc.title,
        };
        store.push(Some(ehr.clone()));

        Ok(ehr)
    }

    pub async fn list_ehr(&self, _ctx: Ctx) -> Result<Vec<Ehr>> {
        let store = self.ehr_store.lock().unwrap();

        let ehrs = store.iter().filter_map(|t| t.clone()).collect();
        Ok(ehrs)
    }

    pub async fn delete_ehr(&self, _ctx: Ctx, id: u64) -> Result<Ehr> {
        let mut store = self.ehr_store.lock().unwrap();

        let ehr = store.get_mut(id as usize).and_then(|t| t.take());

        ehr.ok_or(Error::EhrDeleteFailIdNotFound { id })
    }
}
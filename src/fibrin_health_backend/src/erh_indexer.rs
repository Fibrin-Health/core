use std::collections::HashMap;

use serde::{Serialize};
// use std::collections::HashMap;
use ic_cdk::id;
use candid::{self, CandidType};
use candid::{ Principal, Deserialize };
use uuid::Uuid;


// Upddate comments to natspec



// Struct for the EHR, stored as an NFT
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct EHR {
    pub token_id: u64,         
    pub patient_id: String,    
    pub metadata: String,   
    pub ipfs_link: String,  
    pub owner: Principal,    
    pub compositions: Vec<Composition>,
    pub permitted_users: Vec<Entities>,

    // Add permission arrays
}

// Struct for the medical compositions within an EHR
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Composition {
    pub composition_id: u64,    
    pub title: String,          
    pub content: String,        
    pub timestamp: u64,         
    pub ipfs_link: String,      
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Entities {
    pub entity_id: u64,
    pub entity_type: String,
    pub entity_admin: Principal,
}

#[derive(Default)]
pub struct EHRContract {
    pub ehr_counter: u64, 
    pub ehrs: HashMap<u64, EHR>,  
    pub composition_counter: u64,
}

impl EHRContract {

    pub fn new() -> Self {
        Self {
            ehr_counter: 0,
            ehrs: HashMap::new(),
            composition_counter: 0,
        }
    }

    pub fn create_ehr(&mut self, patient_id: String, metadata: String, ipfs_link: String, owner: Principal) -> EHR {
        let ehr = EHR {
            token_id: self.ehr_counter,
            patient_id,
            metadata,
            ipfs_link,
            owner,
            compositions: Vec::new(),
            permitted_users: Vec::new(),
        };
        self.ehrs.insert(self.ehr_counter, ehr.clone());
        self.ehr_counter += 1;


        // Add uuid


        // integrate NFT functionality



        ehr
    }

    // Add a composition to an existing EHR
    pub fn add_composition(&mut self, ehr_id: u64, title: String, content: String, ipfs_link: String, timestamp: u64) -> Result<(), String> {
        let ehr = self.ehrs.get_mut(&ehr_id).ok_or("EHR not found")?;
        let composition = Composition {
            composition_id: self.composition_counter,
            title,
            content,
            timestamp,
            ipfs_link,
        };
        ehr.compositions.push(composition);
        self.composition_counter += 1;
        Ok(())
    }

    // Get details of an EHR, including compositions
    pub fn get_ehr(&self, ehr_id: u64) -> Result<EHR, String> {
        let ehr = self.ehrs.get(&ehr_id).ok_or("EHR not found")?;
        Ok(ehr.clone())
    }

    pub fn grant_access(&mut self, ehr_id: u64, entity_id: u64, entity_type: String, entity_admin: Principal) -> Result<(), String> {
        let ehr = self.ehrs.get_mut(&ehr_id).ok_or("EHR not found")?;


        // integrate uuid
        if ehr.permitted_users.iter().any(|e| e.entity_id == entity_id && e.entity_admin == entity_admin) {
            return Err("Entity already has access".to_string());
        }
        let entity = Entities {
            entity_id,
            entity_type,
            entity_admin,
        };
        ehr.permitted_users.push(entity);
        Ok(())
    }


}
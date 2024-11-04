use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use uuid::Uuid;
use nostr::prelude::*;
use std::sync::{Arc, Mutex};
use std::error::Error;

// Decentralized Identity (DID) structure
#[derive(Debug, Serialize, Deserialize)]
struct DID {
    id: String,
    public_key: String,
}

impl DID {
    fn new() -> Self {
        Self {
            id: format!("did:example:{}", Uuid::new_v4()),
            public_key: Uuid::new_v4().to_string(),
        }
    }

    fn display_did(&self) {
        println!("DID: {}", self.id);
        println!("Public Key: {}\n", self.public_key);
    }
}

// AI Agent with a unique DID and local storage
#[derive(Debug)]
struct AIAgent {
    did: DID,
    local_storage: Arc<Mutex<HashMap<String, String>>>, // Simulated local storage
}

impl AIAgent {
    fn new() -> Self {
        Self {
            did: DID::new(),
            local_storage: Arc::new(Mutex::new(HashMap::new())),
        }
    }

    // Store data securely in local storage
    fn store_data(&self, key: String, value: String) {
        let mut storage = self.local_storage.lock().unwrap();
        storage.insert(key.clone(), value.clone());
        println!("Data stored successfully: ({}, {})", key, value);
    }

    // Retrieve data from local storage
    fn retrieve_data(&self, key: &str) -> Option<String> {
        let storage = self.local_storage.lock().unwrap();
        storage.get(key).cloned()
    }

    // Communicate with another AI agent over a simulated network (P2P)
    async fn communicate(&self, message: &str) -> Result<(), Box<dyn Error>> {
        let url = "https://httpbin.org/post"; // Simulated endpoint for P2P communication
        let payload = serde_json::json!({
            "from": self.did.id,
            "message": message,
        });
        
        let client = reqwest::Client::new();
        let response = client.post(url).json(&payload).send().await?;

        println!("Message sent: {}", message);
        println!("Response: {:?}", response.text().await?);
        Ok(())
    }
}

#[tokio::main]
async fn main() {
    // Create a new AI agent with a unique DID
    let agent = AIAgent::new();
    agent.did.display_did();

    // Store and retrieve data securely
    let data_key = "sample_data".to_string();
    let data_value = "This is a decentralized storage example.".to_string();
    agent.store_data(data_key.clone(), data_value);

    if let Some(value) = agent.retrieve_data(&data_key) {
        println!("Retrieved Data: {}", value);
    } else {
        println!("Data not found.");
    }

    // Simulate communication with another AI agent
    if let Err(e) = agent.communicate("Hello from Self-Sovereign AI!").await {
        eprintln!("Failed to communicate: {}", e);
    }
}


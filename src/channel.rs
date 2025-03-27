#![warn(missing_docs)]
use std::sync::Arc;

use serde_derive::Deserialize;

use crate::{EslConnection, EslError};
use csv::{ReaderBuilder}; 

#[derive(Debug, Deserialize, Clone)]
pub struct Channel {
    pub uuid: String,
    pub direction: String,
    pub created: String,
    pub created_epoch: String,
    pub name: String,
    pub state: String,
    pub cid_name: String,
    pub cid_num: String,
    pub ip_addr: String,
    pub dest: String,
    pub application: String,
    pub application_data: String,
    pub dialplan: String,
    pub context: String,
    pub read_codec: String,
    pub read_rate: String,
    pub read_bit_rate: String,
    pub write_codec: String,
    pub write_rate: String,
    pub write_bit_rate: String,
    pub secure: String,
    pub hostname: String,
    pub presence_id: String,
    pub presence_data: String,
    pub accountcode: String,
    pub callstate: String,
    pub callee_name: String,
    pub callee_num: String,
    pub callee_direction: String,
    pub call_uuid: String,
    pub sent_callee_name: String,
    pub sent_callee_num: String,
    pub initial_cid_name: String,
    pub initial_cid_num: String,
    pub initial_ip_addr: String,
    pub initial_dest: String,
    pub initial_dialplan: String,
    pub initial_context: String,
}

impl EslConnection {
    pub async fn get_channels(self: &Arc<EslConnection>) -> Result<Vec<Channel>, ()> {
        let response = self.api("show channels").await;

        if response.is_err() {
            println!("Error before everything");
            return Err(());
        }
        let response_clone = response.clone().unwrap(); 
        let clone = response_clone.as_bytes(); 
        
        let mut rdr = ReaderBuilder::new()
            .has_headers(true)
            .from_reader(clone);

        let mut channels = Vec::new();

        let resp = rdr.deserialize::<Channel>();
        for result in resp {
            if result.is_ok() { 
                let mut res: Channel = result.unwrap();
                channels.push(res);
            }
        }
        
        Ok(channels)
    }
}

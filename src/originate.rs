
#[allow(missing_docs)]
use std::sync::Arc;
use crate::Channel;
use crate::EslError;

use crate::EslConnection;


/// Originate enum
#[allow(missing_docs)]
pub enum OriginateErrorCode {
    UNIMPLEMENTED,
    NORMAL_TEMPORARY_FAILURE
}

/// Originate struct
pub struct Originate {
    from: String,
    to: String,
    gateway: String,
    connection: Arc<EslConnection>,
    uuid: Option<String>
}

 impl Originate {
    /// Create new originate object
    pub fn new(connection: Arc<EslConnection>, from: String, to: String, gateway: String) -> Self {
        Self {
            from,
            to,
            gateway,
            connection,
            uuid: None
        }
    }
    /// Get call uuid
    pub fn get_uuid(&self) -> Option<String> {
        return self.uuid.clone();
    }
    /// Async api for awaiting an answer event
    pub async fn wait_for_answer(&self) {
        self.connection.wait_for_event(self.uuid.clone().unwrap(), "CHANNEL_ANSWER".to_string()).await;
    }

    /// Async api for awaiting a hangup event
    pub async fn wait_for_hangup(&self) {
         self.connection.wait_for_event(self.uuid.clone().unwrap(), "CHANNEL_HANGUP".to_string()).await;
     }
 
    /// Bridge two channels
    pub async fn bridge(&self, lega_uuid: String, legb_uuid: String ){
        let command = format!("uuid_bridge {} {}", lega_uuid, legb_uuid);
        eprintln!("command {}", command);
        self.connection.api(command.as_str()).await;
    }

    /// Playback audio file to specific channel
    pub async fn broadcast(&self, uuid: String, path: String ) {
        let command = format!("uuid_broadcast {} {}", uuid, path);
        self.connection.api(command.as_str()).await;
    }

    /// Execute an originate call
    pub async fn execute(&mut self) -> Result<(), OriginateErrorCode> {
        let command = format!(
            "originate {{effective_caller_id_number={}}}sofia/gateway/{}/{} &park()",
            self.from, self.to, self.gateway
        );
    
        let response = self.connection.api(command.as_str()).await;
    
        if let Err(e) = response {
            let error_code = e.to_string();
    
            match error_code.as_str() {
                "NORMAL_TEMPORARY_FAILURE" => return Err(OriginateErrorCode::NORMAL_TEMPORARY_FAILURE),
                _ => return Err(OriginateErrorCode::UNIMPLEMENTED),
            }
        }
        // originate {origination_caller_id_number=1234567890,origination_caller_id_name="Fake Name"}
    //originate {origination_caller_id_number=158765533}sofia/gateway/signalwire/+40740057835 &park()
        self.uuid = Some(response.unwrap().to_string());
        Ok(())
    }

    
}
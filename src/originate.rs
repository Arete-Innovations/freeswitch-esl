use crate::Channel;
use crate::EslConnection;
use crate::EslError;
use std::error::Error as StdError;
#[allow(missing_docs)]
use std::sync::Arc;

/// Originate enum
#[allow(missing_docs)]
pub enum OriginateErrorCode {
    UNIMPLEMENTED,
    UNSPECIFIED,
    NO_ROUTE_TRANSIT_NET,
    NO_ROUTE_DESTINATION,
    CHANNEL_UNACCEPTABLE,
    CALL_AWARDED_DELIVERED,
    NORMAL_CLEARING,
    USER_BUSY,
    NO_USER_RESPONSE,
    NO_ANSWER,
    SUBSCRIBER_ABSENT,
    CALL_REJECTED,
    NUMBER_CHANGED,
    REDIRECTION_TO_NEW_DESTINATION,
    EXCHANGE_ROUTING_ERROR,
    DESTINATION_OUT_OF_ORDER,
    INVALID_NUMBER_FORMAT,
    FACILITY_REJECTED,
    RESPONSE_TO_STATUS_ENQUIRY,
    NORMAL_UNSPECIFIED,
    NORMAL_CIRCUIT_CONGESTION,
    NETWORK_OUT_OF_ORDER,
    NORMAL_TEMPORARY_FAILURE,
    SWITCH_CONGESTION,
    ACCESS_INFO_DISCARDED,
    REQUESTED_CHAN_UNAVAIL,
    PRE_EMPTED,
    FACILITY_NOT_SUBSCRIBED,
    OUTGOING_CALL_BARRED,
    INCOMING_CALL_BARRED,
    BEARERCAPABILITY_NOTAUTH,
    BEARERCAPABILITY_NOTAVAIL,
    SERVICE_UNAVAILABLE,
    BEARERCAPABILITY_NOTIMPL,
    CHAN_NOT_IMPLEMENTED,
    FACILITY_NOT_IMPLEMENTED,
    SERVICE_NOT_IMPLEMENTED,
    INVALID_CALL_REFERENCE,
    INCOMPATIBLE_DESTINATION,
    INVALID_MSG_UNSPECIFIED,
    MANDATORY_IE_MISSING,
    MESSAGE_TYPE_NONEXIST,
    WRONG_MESSAGE,
    IE_NONEXIST,
    INVALID_IE_CONTENTS,
    WRONG_CALL_STATE,
    RECOVERY_ON_TIMER_EXPIRE,
    MANDATORY_IE_LENGTH_ERROR,
    PROTOCOL_ERROR,
    INTERWORKING,
    ORIGINATOR_CANCEL,
    CRASH,
    SYSTEM_SHUTDOWN,
    LOSE_RACE,
    MANAGER_REQUEST,
    BLIND_TRANSFER,
    ATTENDED_TRANSFER,
    ALLOTTED_TIMEOUT,
    USER_CHALLENGE,
    MEDIA_TIMEOUT,
    PICKED_OFF,
    USER_NOT_REGISTERED,
    PROGRESS_TIMEOUT,
    GATEWAY_DOWN,
    UNHANDLED,
}

use std::str::FromStr;

impl FromStr for OriginateErrorCode {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use OriginateErrorCode::*;
        match s {
            "UNSPECIFIED" => Ok(UNSPECIFIED),
            "NO_ROUTE_TRANSIT_NET" => Ok(NO_ROUTE_TRANSIT_NET),
            "NO_ROUTE_DESTINATION" => Ok(NO_ROUTE_DESTINATION),
            "CHANNEL_UNACCEPTABLE" => Ok(CHANNEL_UNACCEPTABLE),
            "CALL_AWARDED_DELIVERED" => Ok(CALL_AWARDED_DELIVERED),
            "NORMAL_CLEARING" => Ok(NORMAL_CLEARING),
            "USER_BUSY" => Ok(USER_BUSY),
            "NO_USER_RESPONSE" => Ok(NO_USER_RESPONSE),
            "NO_ANSWER" => Ok(NO_ANSWER),
            "SUBSCRIBER_ABSENT" => Ok(SUBSCRIBER_ABSENT),
            "CALL_REJECTED" => Ok(CALL_REJECTED),
            "NUMBER_CHANGED" => Ok(NUMBER_CHANGED),
            "REDIRECTION_TO_NEW_DESTINATION" => Ok(REDIRECTION_TO_NEW_DESTINATION),
            "EXCHANGE_ROUTING_ERROR" => Ok(EXCHANGE_ROUTING_ERROR),
            "DESTINATION_OUT_OF_ORDER" => Ok(DESTINATION_OUT_OF_ORDER),
            "INVALID_NUMBER_FORMAT" => Ok(INVALID_NUMBER_FORMAT),
            "FACILITY_REJECTED" => Ok(FACILITY_REJECTED),
            "RESPONSE_TO_STATUS_ENQUIRY" => Ok(RESPONSE_TO_STATUS_ENQUIRY),
            "NORMAL_UNSPECIFIED" => Ok(NORMAL_UNSPECIFIED),
            "NORMAL_CIRCUIT_CONGESTION" => Ok(NORMAL_CIRCUIT_CONGESTION),
            "NETWORK_OUT_OF_ORDER" => Ok(NETWORK_OUT_OF_ORDER),
            "NORMAL_TEMPORARY_FAILURE" => Ok(NORMAL_TEMPORARY_FAILURE),
            "SWITCH_CONGESTION" => Ok(SWITCH_CONGESTION),
            "ACCESS_INFO_DISCARDED" => Ok(ACCESS_INFO_DISCARDED),
            "REQUESTED_CHAN_UNAVAIL" => Ok(REQUESTED_CHAN_UNAVAIL),
            "PRE_EMPTED" => Ok(PRE_EMPTED),
            "FACILITY_NOT_SUBSCRIBED" => Ok(FACILITY_NOT_SUBSCRIBED),
            "OUTGOING_CALL_BARRED" => Ok(OUTGOING_CALL_BARRED),
            "INCOMING_CALL_BARRED" => Ok(INCOMING_CALL_BARRED),
            "BEARERCAPABILITY_NOTAUTH" => Ok(BEARERCAPABILITY_NOTAUTH),
            "BEARERCAPABILITY_NOTAVAIL" => Ok(BEARERCAPABILITY_NOTAVAIL),
            "SERVICE_UNAVAILABLE" => Ok(SERVICE_UNAVAILABLE),
            "BEARERCAPABILITY_NOTIMPL" => Ok(BEARERCAPABILITY_NOTIMPL),
            "CHAN_NOT_IMPLEMENTED" => Ok(CHAN_NOT_IMPLEMENTED),
            "FACILITY_NOT_IMPLEMENTED" => Ok(FACILITY_NOT_IMPLEMENTED),
            "SERVICE_NOT_IMPLEMENTED" => Ok(SERVICE_NOT_IMPLEMENTED),
            "INVALID_CALL_REFERENCE" => Ok(INVALID_CALL_REFERENCE),
            "INCOMPATIBLE_DESTINATION" => Ok(INCOMPATIBLE_DESTINATION),
            "INVALID_MSG_UNSPECIFIED" => Ok(INVALID_MSG_UNSPECIFIED),
            "MANDATORY_IE_MISSING" => Ok(MANDATORY_IE_MISSING),
            "MESSAGE_TYPE_NONEXIST" => Ok(MESSAGE_TYPE_NONEXIST),
            "WRONG_MESSAGE" => Ok(WRONG_MESSAGE),
            "IE_NONEXIST" => Ok(IE_NONEXIST),
            "INVALID_IE_CONTENTS" => Ok(INVALID_IE_CONTENTS),
            "WRONG_CALL_STATE" => Ok(WRONG_CALL_STATE),
            "RECOVERY_ON_TIMER_EXPIRE" => Ok(RECOVERY_ON_TIMER_EXPIRE),
            "MANDATORY_IE_LENGTH_ERROR" => Ok(MANDATORY_IE_LENGTH_ERROR),
            "PROTOCOL_ERROR" => Ok(PROTOCOL_ERROR),
            "INTERWORKING" => Ok(INTERWORKING),
            "ORIGINATOR_CANCEL" => Ok(ORIGINATOR_CANCEL),
            "CRASH" => Ok(CRASH),
            "SYSTEM_SHUTDOWN" => Ok(SYSTEM_SHUTDOWN),
            "LOSE_RACE" => Ok(LOSE_RACE),
            "MANAGER_REQUEST" => Ok(MANAGER_REQUEST),
            "BLIND_TRANSFER" => Ok(BLIND_TRANSFER),
            "ATTENDED_TRANSFER" => Ok(ATTENDED_TRANSFER),
            "ALLOTTED_TIMEOUT" => Ok(ALLOTTED_TIMEOUT),
            "USER_CHALLENGE" => Ok(USER_CHALLENGE),
            "MEDIA_TIMEOUT" => Ok(MEDIA_TIMEOUT),
            "PICKED_OFF" => Ok(PICKED_OFF),
            "USER_NOT_REGISTERED" => Ok(USER_NOT_REGISTERED),
            "PROGRESS_TIMEOUT" => Ok(PROGRESS_TIMEOUT),
            "GATEWAY_DOWN" => Ok(GATEWAY_DOWN),
            _ => Err(()),
        }
    }
}

#[allow(missing_docs)]
#[derive(Debug)]
pub enum BridgeError {
    ChannelNotFound,
    InvalidArguments,
    BridgeFailure,
    ChannelDestroyed,
    ProtocolMismatch,
    MediaError,
    PermissionDenied,
    Timeout,
    EslError(EslError),
    Unimplemented,
}

impl FromStr for BridgeError {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "CHAN_NOT_FOUND" | "NO_SUCH_CHANNEL" => Ok(BridgeError::ChannelNotFound),
            "INVALID_ARGS" => Ok(BridgeError::InvalidArguments),
            "BRIDGE_FAILED" => Ok(BridgeError::BridgeFailure),
            "DESTROYED" => Ok(BridgeError::ChannelDestroyed),
            "PROTOCOL_MISMATCH" => Ok(BridgeError::ProtocolMismatch),
            "MEDIA_ERROR" => Ok(BridgeError::MediaError),
            "NO_PERMISSION" => Ok(BridgeError::PermissionDenied),
            "TIMEOUT" => Ok(BridgeError::Timeout),
            _ => Err(()),
        }
    }
}

impl From<EslError> for BridgeError {
    fn from(err: EslError) -> Self {
        BridgeError::EslError(err)
    }
}
/// Originate struct
pub struct Originate {
    from: String,
    to: String,
    gateway: String,
    connection: Arc<EslConnection>,
    uuid: Option<String>,
}

impl Originate {
    /// Create new originate object
    pub fn new(connection: Arc<EslConnection>, from: String, to: String, gateway: String) -> Self {
        Self {
            from,
            to,
            gateway,
            connection,
            uuid: None,
        }
    }
    /// Get call uuid
    pub fn get_uuid(&self) -> Option<String> {
        return self.uuid.clone();
    }
    /// Async api for awaiting an answer event
    pub async fn wait_for_answer(&self) {
        self.connection
            .wait_for_event(self.uuid.clone().unwrap(), "CHANNEL_ANSWER".to_string())
            .await;
    }

    /// Async api for awaiting a hangup event
    pub async fn wait_for_hangup(&self) {
        self.connection
            .wait_for_event(self.uuid.clone().unwrap(), "CHANNEL_HANGUP".to_string())
            .await;
    }

    /// Bridge two channels
    pub async fn bridge(&self, lega_uuid: String, legb_uuid: String) -> Result<(), BridgeError> {
        let cmd = format!("uuid_bridge {} {}", lega_uuid, legb_uuid);
        let response = self.connection.api(&cmd).await?;

        match response.as_str().trim() {
            "+OK" => Ok(()),
            e => {
                let err_str = e.to_string();
                if let Ok(mapped) = BridgeError::from_str(&err_str) {
                    Err(mapped)
                } else {
                    Err(BridgeError::Unimplemented)
                }
            }
        }
    }

    /// Playback audio file to specific channel
    pub async fn broadcast(&self, uuid: String, path: String) {
        let command = format!("uuid_broadcast {} {}", uuid, path);
        self.connection.api(command.as_str()).await;
    }

    /// Execute an originate call
    pub async fn execute(&mut self) -> Result<String, OriginateErrorCode> {
        let command = format!(
            "originate {{effective_caller_id_number={}}}sofia/gateway/{}/{} &park()",
            self.from, self.to, self.gateway
        );

        let response = self.connection.api(command.as_str()).await;

        match response {
            Ok(uuid) => {
                self.uuid = Some(uuid.to_string());

                Ok(uuid)
            }
            Err(e) => {
                let err_str = e.to_string();
                if let Ok(mapped) = OriginateErrorCode::from_str(&err_str) {
                    Err(mapped)
                } else {
                    Err(OriginateErrorCode::UNIMPLEMENTED)
                }
            }
        }
    }
}

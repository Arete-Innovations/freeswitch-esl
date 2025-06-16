use crate::Channel;
use crate::EslConnection;
use crate::EslError;
use std::error::Error as StdError;
#[allow(missing_docs)]
use std::sync::Arc;

/// Originate enum
#[allow(missing_docs)]
pub enum OriginateErrorCode {
    Unimplemented,
    Unspecified,
    NoRouteTransitNet,
    NoRouteDestination,
    ChannelUnacceptable,
    CallAwardedDelivered,
    NormalClearing,
    UserBusy,
    NoUserResponse,
    NoAnswer,
    SubscriberAbsent,
    CallRejected,
    NumberChanged,
    RedirectionToNewDestination,
    ExchangeRoutingError,
    DestinationOutOfOrder,
    InvalidNumberFormat,
    FacilityRejected,
    ResponseToStatusEnquiry,
    NormalUnspecified,
    NormalCircuitCongestion,
    NetworkOutOfOrder,
    NormalTemporaryFailure,
    SwitchCongestion,
    AccessInfoDiscarded,
    RequestedChanUnavail,
    PreEmpted,
    FacilityNotSubscribed,
    OutgoingCallBarred,
    IncomingCallBarred,
    BearercapabilityNotauth,
    BearercapabilityNotavail,
    ServiceUnavailable,
    BearercapabilityNotimpl,
    ChanNotImplemented,
    FacilityNotImplemented,
    ServiceNotImplemented,
    InvalidCallReference,
    IncompatibleDestination,
    InvalidMsgUnspecified,
    MandatoryIeMissing,
    MessageTypeNonexist,
    WrongMessage,
    IeNonexist,
    InvalidIeContents,
    WrongCallState,
    RecoveryOnTimerExpire,
    MandatoryIeLengthError,
    ProtocolError,
    Interworking,
    OriginatorCancel,
    Crash,
    SystemShutdown,
    LoseRace,
    ManagerRequest,
    BlindTransfer,
    AttendedTransfer,
    AllottedTimeout,
    UserChallenge,
    MediaTimeout,
    PickedOff,
    UserNotRegistered,
    ProgressTimeout,
    GatewayDown,
    Unhandled,
}


use std::str::FromStr;

impl FromStr for OriginateErrorCode {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use OriginateErrorCode::*;
        match s {
            "UNIMPLEMENTED" => Ok(Unimplemented),
            "UNSPECIFIED" => Ok(Unspecified),
            "NO_ROUTE_TRANSIT_NET" => Ok(NoRouteTransitNet),
            "NO_ROUTE_DESTINATION" => Ok(NoRouteDestination),
            "CHANNEL_UNACCEPTABLE" => Ok(ChannelUnacceptable),
            "CALL_AWARDED_DELIVERED" => Ok(CallAwardedDelivered),
            "NORMAL_CLEARING" => Ok(NormalClearing),
            "USER_BUSY" => Ok(UserBusy),
            "NO_USER_RESPONSE" => Ok(NoUserResponse),
            "NO_ANSWER" => Ok(NoAnswer),
            "SUBSCRIBER_ABSENT" => Ok(SubscriberAbsent),
            "CALL_REJECTED" => Ok(CallRejected),
            "NUMBER_CHANGED" => Ok(NumberChanged),
            "REDIRECTION_TO_NEW_DESTINATION" => Ok(RedirectionToNewDestination),
            "EXCHANGE_ROUTING_ERROR" => Ok(ExchangeRoutingError),
            "DESTINATION_OUT_OF_ORDER" => Ok(DestinationOutOfOrder),
            "INVALID_NUMBER_FORMAT" => Ok(InvalidNumberFormat),
            "FACILITY_REJECTED" => Ok(FacilityRejected),
            "RESPONSE_TO_STATUS_ENQUIRY" => Ok(ResponseToStatusEnquiry),
            "NORMAL_UNSPECIFIED" => Ok(NormalUnspecified),
            "NORMAL_CIRCUIT_CONGESTION" => Ok(NormalCircuitCongestion),
            "NETWORK_OUT_OF_ORDER" => Ok(NetworkOutOfOrder),
            "NORMAL_TEMPORARY_FAILURE" => Ok(NormalTemporaryFailure),
            "SWITCH_CONGESTION" => Ok(SwitchCongestion),
            "ACCESS_INFO_DISCARDED" => Ok(AccessInfoDiscarded),
            "REQUESTED_CHAN_UNAVAIL" => Ok(RequestedChanUnavail),
            "PRE_EMPTED" => Ok(PreEmpted),
            "FACILITY_NOT_SUBSCRIBED" => Ok(FacilityNotSubscribed),
            "OUTGOING_CALL_BARRED" => Ok(OutgoingCallBarred),
            "INCOMING_CALL_BARRED" => Ok(IncomingCallBarred),
            "BEARERCAPABILITY_NOTAUTH" => Ok(BearercapabilityNotauth),
            "BEARERCAPABILITY_NOTAVAIL" => Ok(BearercapabilityNotavail),
            "SERVICE_UNAVAILABLE" => Ok(ServiceUnavailable),
            "BEARERCAPABILITY_NOTIMPL" => Ok(BearercapabilityNotimpl),
            "CHAN_NOT_IMPLEMENTED" => Ok(ChanNotImplemented),
            "FACILITY_NOT_IMPLEMENTED" => Ok(FacilityNotImplemented),
            "SERVICE_NOT_IMPLEMENTED" => Ok(ServiceNotImplemented),
            "INVALID_CALL_REFERENCE" => Ok(InvalidCallReference),
            "INCOMPATIBLE_DESTINATION" => Ok(IncompatibleDestination),
            "INVALID_MSG_UNSPECIFIED" => Ok(InvalidMsgUnspecified),
            "MANDATORY_IE_MISSING" => Ok(MandatoryIeMissing),
            "MESSAGE_TYPE_NONEXIST" => Ok(MessageTypeNonexist),
            "WRONG_MESSAGE" => Ok(WrongMessage),
            "IE_NONEXIST" => Ok(IeNonexist),
            "INVALID_IE_CONTENTS" => Ok(InvalidIeContents),
            "WRONG_CALL_STATE" => Ok(WrongCallState),
            "RECOVERY_ON_TIMER_EXPIRE" => Ok(RecoveryOnTimerExpire),
            "MANDATORY_IE_LENGTH_ERROR" => Ok(MandatoryIeLengthError),
            "PROTOCOL_ERROR" => Ok(ProtocolError),
            "INTERWORKING" => Ok(Interworking),
            "ORIGINATOR_CANCEL" => Ok(OriginatorCancel),
            "CRASH" => Ok(Crash),
            "SYSTEM_SHUTDOWN" => Ok(SystemShutdown),
            "LOSE_RACE" => Ok(LoseRace),
            "MANAGER_REQUEST" => Ok(ManagerRequest),
            "BLIND_TRANSFER" => Ok(BlindTransfer),
            "ATTENDED_TRANSFER" => Ok(AttendedTransfer),
            "ALLOTTED_TIMEOUT" => Ok(AllottedTimeout),
            "USER_CHALLENGE" => Ok(UserChallenge),
            "MEDIA_TIMEOUT" => Ok(MediaTimeout),
            "PICKED_OFF" => Ok(PickedOff),
            "USER_NOT_REGISTERED" => Ok(UserNotRegistered),
            "PROGRESS_TIMEOUT" => Ok(ProgressTimeout),
            "GATEWAY_DOWN" => Ok(GatewayDown),
            "UNHANDLED" => Ok(Unhandled),
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
                    Err(OriginateErrorCode::Unimplemented)
                }
            }
        }
    }
}

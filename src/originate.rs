use serde_derive::Deserialize;
use serde_derive::Serialize;

use crate::Channel;
use crate::EslConnection;
use crate::EslError;
use std::error::Error as StdError;
use std::fmt;

use std::str::FromStr;

#[allow(missing_docs)]
use std::sync::Arc;

/// Originate enum
#[allow(missing_docs)]
#[derive(Debug, Clone, Serialize, Deserialize)]
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
    InvalidUrl,
    Unhandled,
}

impl fmt::Display for OriginateErrorCode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        use OriginateErrorCode::*;
        let s = match self {
            Unimplemented => "Unimplemented",
            Unspecified => "Unspecified",
            NoRouteTransitNet => "No Route Transit Net",
            NoRouteDestination => "No Route Destination",
            ChannelUnacceptable => "Channel Unacceptable",
            CallAwardedDelivered => "Call Awarded Delivered",
            NormalClearing => "Normal Clearing",
            UserBusy => "User Busy",
            NoUserResponse => "No User Response",
            NoAnswer => "No Answer",
            SubscriberAbsent => "Subscriber Absent",
            CallRejected => "Call Rejected",
            NumberChanged => "Number Changed",
            RedirectionToNewDestination => "Redirection To New Destination",
            ExchangeRoutingError => "Exchange Routing Error",
            DestinationOutOfOrder => "Destination Out Of Order",
            InvalidNumberFormat => "Invalid Number Format",
            FacilityRejected => "Facility Rejected",
            ResponseToStatusEnquiry => "Response To Status Enquiry",
            NormalUnspecified => "Normal Unspecified",
            NormalCircuitCongestion => "Normal Circuit Congestion",
            NetworkOutOfOrder => "Network Out Of Order",
            NormalTemporaryFailure => "Normal Temporary Failure",
            SwitchCongestion => "Switch Congestion",
            AccessInfoDiscarded => "Access Info Discarded",
            RequestedChanUnavail => "Requested Channel Unavailable",
            PreEmpted => "Pre-Empted",
            FacilityNotSubscribed => "Facility Not Subscribed",
            OutgoingCallBarred => "Outgoing Call Barred",
            IncomingCallBarred => "Incoming Call Barred",
            BearercapabilityNotauth => "Bearer capability Not authorized",
            BearercapabilityNotavail => "Bearer capability Not available",
            ServiceUnavailable => "Service Unavailable",
            BearercapabilityNotimpl => "Bearer capability Not implemented",
            ChanNotImplemented => "Channel Not Implemented",
            FacilityNotImplemented => "Facility Not Implemented",
            ServiceNotImplemented => "Service Not Implemented",
            InvalidCallReference => "Invalid Call Reference",
            IncompatibleDestination => "Incompatible Destination",
            InvalidMsgUnspecified => "Invalid Message Unspecified",
            MandatoryIeMissing => "Mandatory IE Missing",
            MessageTypeNonexist => "Message Type Nonexistent",
            WrongMessage => "Wrong Message",
            IeNonexist => "IE Nonexistent",
            InvalidIeContents => "Invalid IE Contents",
            WrongCallState => "Wrong Call State",
            RecoveryOnTimerExpire => "Recovery On Timer Expire",
            MandatoryIeLengthError => "Mandatory IE Length Error",
            ProtocolError => "Protocol Error",
            Interworking => "Interworking",
            OriginatorCancel => "Originator Cancel",
            Crash => "Crash",
            SystemShutdown => "System Shutdown",
            LoseRace => "Lose Race",
            ManagerRequest => "Manager Request",
            BlindTransfer => "Blind Transfer",
            AttendedTransfer => "Attended Transfer",
            AllottedTimeout => "Allotted Timeout",
            UserChallenge => "User Challenge",
            MediaTimeout => "Media Timeout",
            PickedOff => "Picked Off",
            UserNotRegistered => "User Not Registered",
            ProgressTimeout => "Progress Timeout",
            GatewayDown => "Gateway Down",
            InvalidUrl => "Invalid Url",
            Unhandled => "Unhandled",
        };
        write!(f, "{}", s)
    }
}

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
            "INVALID_URL" => Ok(InvalidUrl),
            "UNHANDLED" => Ok(Unhandled),
            _ => Err(()),
        }
    }
}

#[allow(missing_docs)]
#[derive(Debug, Clone, Serialize, Deserialize)]
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

impl fmt::Display for BridgeError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            BridgeError::ChannelNotFound => write!(f, "Channel Not Found"),
            BridgeError::InvalidArguments => write!(f, "Invalid Arguments"),
            BridgeError::BridgeFailure => write!(f, "Bridge Failure"),
            BridgeError::ChannelDestroyed => write!(f, "Channel Destroyed"),
            BridgeError::ProtocolMismatch => write!(f, "Protocol Mismatch"),
            BridgeError::MediaError => write!(f, "Media Error"),
            BridgeError::PermissionDenied => write!(f, "Permission Denied"),
            BridgeError::Timeout => write!(f, "Timeout"),
            BridgeError::EslError(e) => write!(f, "ESL Error: {}", e),
            BridgeError::Unimplemented => write!(f, "Unimplemented"),
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
    uuid: Option<String>,
}

impl Originate {
impl Originate {
    /// Create new originate object
    pub fn new(connection: Arc<EslConnection>, from: String, to: String, gateway: String) -> Self {
        Self {
            from,
            to,
            gateway,
            connection,
            uuid: None,
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
    pub async fn broadcast(&self, uuid: String, path: String) {
        let command = format!("uuid_broadcast {} {}", uuid, path);
        self.connection.api(command.as_str()).await;
    }

    /// Execute an originate call
    pub async fn execute(&mut self) -> Result<String, OriginateErrorCode> {
    pub async fn execute(&mut self) -> Result<String, OriginateErrorCode> {
        let command = format!(
            "originate {{effective_caller_id_number={}}}sofia/gateway/{}/{} &park()",
            self.from, self.gateway, self.to
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

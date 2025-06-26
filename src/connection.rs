#![warn(missing_docs)]

use crate::code::{Code, ParseCode};
use crate::error::EslError;
use crate::esl::EslConnectionType;
use crate::io::EslCodec;
use crate::{EventHandler, EventManager};
use futures::SinkExt;
use serde::de::DeserializeOwned;
use serde_json::Value;
use std::collections::{HashMap, VecDeque};
use std::sync::atomic::Ordering;
use std::sync::{atomic::AtomicBool, Arc};
use tokio::io::WriteHalf;
use tokio::net::TcpStream;
use tokio::sync::{
    oneshot::{channel, Sender},
    Mutex,
};
use crate::event::Event;

use tokio_stream::StreamExt;
use tokio_util::codec::{FramedRead, FramedWrite};
use tracing::trace;

struct EventAwait(String, String);


/// contains Esl connection with freeswitch
#[derive(Debug)]
pub struct EslConnection {
    password: String,
    commands: Arc<Mutex<VecDeque<Sender<Event>>>>,
    transport_tx: Arc<Mutex<FramedWrite<WriteHalf<TcpStream>, EslCodec>>>,
    background_jobs: Arc<Mutex<HashMap<String, Sender<Event>>>>,
    wait_for_events: Arc<Mutex<HashMap<String, Sender<Event>>>>,
    connected: AtomicBool,
    pub(crate) call_uuid: Option<String>,
    connection_info: Option<HashMap<String, Value>>,
    event_manager: Arc<Mutex<EventManager>>

}

impl EslConnection {
    /// Returns one of the session parameters as a string
    pub fn get_info_string(&self, key: &str) -> Option<String> {
        let value = self.connection_info.as_ref()?.get(key)?.clone();
        serde_json::from_value(value).ok()?
    }

    /// Returns one of the session parameters as any deserializable type
    pub fn get_info<V: DeserializeOwned>(&self, key: &str) -> Option<V> {
        let value = self.connection_info.as_ref()?.get(key)?.clone();
        serde_json::from_value(value).ok()?
    }
    /// Bind a callback event
    pub async  fn bind_event(&self, event: String, handler: EventHandler) {
        let mut manager = self.event_manager.lock().await;
        manager.register_handler(event, handler);
    }
    
    /// returns call uuid in outbound mode
    pub async fn call_uuid(&self) -> Option<String> {
        self.call_uuid.clone()
    }
    /// disconnects from freeswitch
    pub async fn disconnect(self) -> Result<(), EslError> {
        self.send_recv(b"exit").await?;
        self.connected.store(false, Ordering::Relaxed);
        Ok(())
    }
    /// returns status of esl connection
    pub fn connected(&self) -> bool {
        self.connected.load(Ordering::Relaxed)
    }

    // send raw meesage without freeswitch response
    pub async fn send(&self, item: &[u8]) -> Result<(), EslError> {
        let mut transport = self.transport_tx.lock().await;
        transport.send(item).await
    }
    /// sends raw message to freeswitch and receives reply
    pub async fn send_recv(&self, item: &[u8]) -> Result<Event, EslError> {
        self.send(item).await?;
        let (tx, rx) = channel();
        self.commands.lock().await.push_back(tx);
        Ok(rx.await?)
    }

    pub async fn wait_for_event(&self, waiter: String, event: String) {
        let (tx, rx) = channel();

        let mut hash = event.clone();
        hash.push_str(&waiter);

        println!("hash inserted {}", hash.clone());

        self.wait_for_events.lock().await.insert(hash.clone(), tx);

        let resp = rx.await;
    }

    pub(crate) async fn new(
        stream: TcpStream,
        password: impl ToString,
        connection_type: EslConnectionType,
    ) -> Result<Self, EslError> {
        // let sender = Arc::new(sender);
        let commands = Arc::new(Mutex::new(VecDeque::new()));
        let inner_commands = Arc::clone(&commands);
        let background_jobs = Arc::new(Mutex::new(HashMap::new()));
        let wait_for_events = Arc::new(Mutex::new(HashMap::new()));
        let inner_background_jobs = Arc::clone(&background_jobs);
        let inner_wait_for_events_jobs = Arc::clone(&wait_for_events);
        let event_manager = Arc::new(Mutex::new(EventManager::new()));
        let event_manager_inner = event_manager.clone();
        let esl_codec = EslCodec {};
        let (read_half, write_half) = tokio::io::split(stream);
        let mut transport_rx = FramedRead::new(read_half, esl_codec.clone());
        let transport_tx = Arc::new(Mutex::new(FramedWrite::new(write_half, esl_codec.clone())));
        if connection_type == EslConnectionType::Inbound {
            transport_rx.next().await;
        }
        let mut connection = Self {
            password: password.to_string(),
            commands,
            background_jobs,
            wait_for_events,
            transport_tx,
            connected: AtomicBool::new(false),
            call_uuid: None,
            connection_info: None,
            event_manager: event_manager
        };
        tokio::spawn(async move {
            loop {
                if let Some(Ok(event)) = transport_rx.next().await {
                    if let Some(event_type) = event.headers.get("Content-Type") {
                        match event_type.as_str().unwrap() {
                            "text/disconnect-notice" => {
                                trace!("got disconnect notice");
                                return;
                            }
                            "text/event-json" => {
                         
                                let data = event
                                    .body()
                                    .clone()
                                    .expect("Unable to get body of event-json");

                                let event_body = parse_json_body(&data)
                                    .expect("Unable to parse body of event-json");
                                let job_uuid = event_body.get("Job-UUID");

                        
                                if event_body.contains_key("Event-Name")
                                    && event_body.contains_key("Unique-ID")
                                {

                                    let event_name = event_body.get("Event-Name");
                                    let name = event_name.unwrap().as_str().unwrap().to_string();
                                    let unique_id =
                                        event_body.get("Unique-ID").unwrap().as_str().unwrap();
                          
                                    let mut hash = name.clone();
                                    hash.push_str(unique_id);


                                    {
                                        event_manager_inner.lock().await.trigger_event(name, &event_body).await;
                                    }

                                    if let Some(tx) = inner_wait_for_events_jobs.lock().await.remove(&hash) {
                                        tx.send(event.clone()).expect("Unable to send message.");
                                    }

                                    
                                }

                                if let Some(job_uuid) = job_uuid {
                                    let job_uuid = job_uuid.as_str().unwrap();
                                    if let Some(tx) =
                                        inner_background_jobs.lock().await.remove(job_uuid)
                                    {
                                        tx.send(event.clone())
                                            .expect("Unable to send channel message from bgapi");
                                    }

                                    trace!("continued");
                                    continue;
                                }
                                if let Some(application_uuid) = event_body.get("Application-UUID") {
                                    let job_uuid = application_uuid.as_str().unwrap();
                                    if let Some(event_name) = event_body.get("Event-Name") {
                                        if let Some(event_name) = event_name.as_str() {
                                            if event_name == "CHANNEL_EXECUTE_COMPLETE" {
                                                if let Some(tx) = inner_background_jobs
                                                    .lock()
                                                    .await
                                                    .remove(job_uuid)
                                                {
                                                    tx.send(event).expect(
                                                        "Unable to send channel message from bgapi",
                                                    );
                                                }
                                         
                                            }
                                        }
                                    }
                                }
                              
                                continue;
                            }
                            _ => {
                                trace!("got another event {:?}", event);
                            }
                        }
                    }
                    if let Some(tx) = inner_commands.lock().await.pop_front() {
                        tx.send(event).expect("msg");
                    }
                }
            }
        });
        match connection_type {
            EslConnectionType::Inbound => {
                let auth_response = connection.auth().await?;
                
                connection
                    .subscribe(vec!["BACKGROUND_JOB", "CHANNEL_EXECUTE_COMPLETE"])
                    .await?;
            }
            EslConnectionType::Outbound => {
                let response = connection.send_recv(b"connect").await?;
                trace!("{:?}", response);
                connection.connection_info = Some(response.headers().clone());
                let response = connection
                    .subscribe(vec!["BACKGROUND_JOB", "CHANNEL_EXECUTE_COMPLETE"])
                    .await?;
                trace!("{:?}", response);
                let response = connection.send_recv(b"myevents").await?;
                trace!("{:?}", response);
                let connection_info = connection.connection_info.as_ref().unwrap();

                let channel_unique_id = connection_info
                    .get("Channel-Unique-ID")
                    .unwrap()
                    .as_str()
                    .unwrap();
                connection.call_uuid = Some(channel_unique_id.to_string());
            }
        }
        Ok(connection)
    }

    /// subscribes to given events
    pub async fn subscribe(&self, events: Vec<&str>) -> Result<Event, EslError> {
        let message = format!("event json {}", events.join(" "));
        self.send_recv(message.as_bytes()).await
    }

    pub(crate) async fn auth(&self) -> Result<String, EslError> {
        let auth_response = self
            .send_recv(format!("auth {}", self.password).as_bytes())
            .await?;
        let auth_headers = auth_response.headers();
        let reply_text = auth_headers.get("Reply-Text").ok_or_else(|| {
            EslError::InternalError("Reply-Text in auth request was not found".into())
        })?;
        let reply_text = reply_text.as_str().unwrap();
        let (code, text) = parse_api_response(reply_text)?;
        match code {
            Code::Ok => {
                self.connected.store(true, Ordering::Relaxed);
                Ok(text)
            }
            Code::Err => Err(EslError::AuthFailed),
            Code::Unknown => Err(EslError::InternalError(
                "Got unknown code in auth request".into(),
            )),
        }
    }

    /// For hanging up call in outbound mode
    pub async fn hangup(&self, reason: &str) -> Result<Event, EslError> {
        self.execute("hangup", reason).await
    }

    /// executes application in freeswitch
    pub async fn execute(&self, app_name: &str, app_args: &str) -> Result<Event, EslError> {
        let event_uuid = uuid::Uuid::new_v4().to_string();
        let (tx, rx) = channel();
        self.background_jobs
            .lock()
            .await
            .insert(event_uuid.clone(), tx);
        let call_uuid = self.call_uuid.as_ref().unwrap().clone();
        let command  = format!("sendmsg {}\nexecute-app-name: {}\nexecute-app-arg: {}\ncall-command: execute\nEvent-UUID: {}",call_uuid,app_name,app_args,event_uuid);
        let response = self.send_recv(command.as_bytes()).await?;
        trace!("inside execute {:?}", response);
        let resp = rx.await?;
        trace!("got response from channel {:?}", resp);
        Ok(resp)
    }

    /// answers call in outbound mode
    pub async fn answer(&self) -> Result<Event, EslError> {
        self.execute("answer", "").await
    }

    /// sends api command to freeswitch
    pub async fn api(&self, command: &str) -> Result<String, EslError> {
        let response = self.send_recv(format!("api {}", command).as_bytes()).await;
        let event = response?;
        let body = event
            .body
            .ok_or_else(|| EslError::InternalError("Didnt get body in api response".into()))?;

        let (code, text) = parse_api_response(&body)?;
        match code {
            Code::Ok => Ok(text),
            Code::Err => Err(EslError::ApiError(text)),
            Code::Unknown => Ok(body),
        }
    }

    /// sends bgapi commands to freeswitch
    pub async fn bgapi(&self, command: &str) -> Result<String, EslError> {
        trace!("Send bgapi {}", command);
        let job_uuid = uuid::Uuid::new_v4().to_string();
        let (tx, rx) = channel();
        self.background_jobs
            .lock()
            .await
            .insert(job_uuid.clone(), tx);

        self.send_recv(format!("bgapi {}\nJob-UUID: {}", command, job_uuid).as_bytes())
            .await?;

        let resp = rx.await?;
        let body = resp
            .body()
            .clone()
            .ok_or_else(|| EslError::InternalError("body was not found in event/json".into()))?;

        let body_hashmap = parse_json_body(&body)?;

        let mut hsmp = resp.headers().clone();
        hsmp.extend(body_hashmap);
        let body = hsmp
            .get("_body")
            .ok_or_else(|| EslError::InternalError("body was not found in event/json".into()))?;
        let body = body.as_str().unwrap();
        let (code, text) = parse_api_response(body)?;
        match code {
            Code::Ok => Ok(text),
            Code::Err => Err(EslError::ApiError(text)),
            Code::Unknown => Ok(body.to_string()),
        }
    }
}
fn parse_api_response(body: &str) -> Result<(Code, String), EslError> {
    let space_index = body
        .find(char::is_whitespace)
        .ok_or_else(|| EslError::InternalError("Unable to find space index".into()))?;
    let code = &body[..space_index];
    let text_start = space_index + 1;
    let body_length = body.len();
    let text = if text_start < body_length {
        body[text_start..(body_length - 1)].to_string()
    } else {
        String::new()
    };
    let code = code.parse_code()?;
    Ok((code, text))
}
fn parse_json_body(body: &str) -> Result<HashMap<String, Value>, EslError> {
    Ok(serde_json::from_str(body)?)
}

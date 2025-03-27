use std::{collections::HashMap, sync::Arc};
use serde_json::Value;
use freeswitch_esl::{Esl, EslConnection, EslError, EventHandler, Originate, OriginateErrorCode};
use tokio::net::TcpStream;



async fn launch_call(
    connection: Arc<EslConnection>,
    from: String,
    to: String,
) -> Result<Originate, OriginateErrorCode> {
    let mut originate = Originate::new(connection, from, to, "itsp".to_string());
    
    originate.execute().await?;
    Ok(originate) 
}

#[tokio::main]
async fn main() -> Result<(), EslError> {
    let addr = "localhost:8021"; // Freeswitch host
    let stream = TcpStream::connect(addr).await?;
    let password = "ClueCon";
    let inbound = Arc::new(Esl::inbound(stream, password).await?);

    inbound
            .subscribe(vec![
                "CHANNEL_ANSWER",
                "CHANNEL_CALLSTATE",
                "CHANNEL_HANGUP_COMPLETE",
                "CHANNEL_BRIDGE",
                "CHANNEL_HANGUP",
            ])
            .await
            .unwrap();
        
    let reloadxml = inbound.api("reloadxml").await?;
    println!("reloadxml response : {:?}", reloadxml);

    let reloadxml = inbound.bgapi("reloadxml").await?;
    println!("reloadxml response : {:?}", reloadxml);


    // event driver mechanism
    let handle_answer: EventHandler = Box::new(move |event_body: &HashMap<String, Value>| {
        let event_body = event_body.clone(); // Clone the event_body to avoid lifetime issues
        Box::pin(async move {
            // You can access the body safely here
            let name = event_body
                        .get("Caller-Caller-ID-Name")
                        .unwrap()
                        .as_str()
                        .unwrap()
                        .to_string();
            println!("Call answer from: {}", name);
        })
    });

    inbound
        .bind_event("CHANNEL_ANSWER".to_string(), handle_answer)
        .await;

    // async
    let result = launch_call(inbound, "999".to_string(), "222".to_string()).await;
    if let Ok(call) = result {
        call.wait_for_answer().await;

        call.wait_for_hangup().await;
    }

    
    Ok(())
}

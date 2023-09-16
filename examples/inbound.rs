use freeswitch_esl::{Esl, EslError};
use tokio::net::TcpStream;

#[tokio::main]
async fn main() -> Result<(), EslError> {
    let addr = "localhost:8021"; // Freeswitch host
    let password = "ClueCon";
    let stream = TcpStream::connect(addr).await?;
    let inbound = Esl::inbound(stream, password).await?;

    let reloadxml = inbound.api("reloadxml").await?;
    println!("reloadxml response : {:?}", reloadxml);

    let reloadxml = inbound.bgapi("reloadxml").await?;
    println!("reloadxml response : {:?}", reloadxml);

    Ok(())
}

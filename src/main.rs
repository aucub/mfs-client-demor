use rsocket_rust::extension::{CompositeMetadata, MimeType, RoutingMetadata};
use rsocket_rust::prelude::*;
use rsocket_rust::utils::{EchoRSocket, Writeable};
use rsocket_rust::{Error, Result};
use rsocket_rust_messaging::Requester;
use rsocket_rust_transport_tcp::TcpClientTransport;
use crate::token::TOKENMetadata;

mod token;

#[tokio::main]
async fn main() -> Result<()> {
    let token=String::from("eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.eyJqdGkiOiJhYWExNjA5Ny0wNzI4LTQ1YTItYTQ1MC1jMTMxMGJjMmQ0OTEiLCJpc3MiOiIwYzU5OTg5ZDM5NzAzODBhZTE2ODg4MDY4NmM0YTA3MCIsInN1YiI6IjBjNTk5ODlkMzk3MDM4MGFlMTY4ODgwNjg2YzRhMDcwIiwiZXhwIjoxNjg0MjUwNDk3LCJhdWQiOiJtZnMiLCJzY29wZSI6WyJ1c2VyTWFuIiwiZ2VuZXJhdGVKd3QiLCJzZWFyY2hPbmxpbmUiLCJyb2xlIiwiY29ubmVjdCIsInB1c2giLCJwdWJsaXNoIiwiY29uc3VtZSIsInF1ZXJ5Il19.2hfCRCUaTjR1HRLren6dfZ7LwQ3Z2uw__-_lvwEZP9M");
    let routing = RoutingMetadata::builder().push("connect".into()).build();
    let authentication=TOKENMetadata::builder().push(token).build();
    let cm = CompositeMetadata::builder()
        .push(MimeType::MESSAGE_X_RSOCKET_ROUTING_V0, routing.bytes())
        .push(MimeType::from("message/x.rsocket.authentication.v0"), authentication.bytes())
        .build();
    let setup = Payload::builder().set_metadata(cm).build();
    let cli = RSocketFactory::connect()
        .setup(setup)
        .metadata_mime_type(MimeType::MESSAGE_X_RSOCKET_COMPOSITE_METADATA_V0)
        .transport(TcpClientTransport::from("127.0.0.1:9898"))
        .start()
        .await?;
    let rsocket: Box<dyn RSocket> = Box::new(cli);
    let requester = Requester::from(rsocket);
    /*requester.route("consume")
        .data("{'queueType':'classic','queue':'test2','manual':'false'}")
        .retrieve_flux();*/
    println!("OK");
    Ok(())
}
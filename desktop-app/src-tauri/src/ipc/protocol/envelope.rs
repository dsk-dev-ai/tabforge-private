use serde::{Deserialize, Serialize};
use serde_json::Value;

pub const PROTOCOL_VERSION:&str="v1";

#[derive(
    Debug,
    Clone,
    Serialize,
    Deserialize
)]

#[serde(rename_all="camelCase")]

pub struct EnvelopeV1{

    pub version:String,

    pub trace_id:String,

    pub timestamp:u64,

    pub session_id:String,

    #[serde(rename="type")]

    pub message_type:MessageType,

    pub payload:Value
}


#[derive(
    Debug,
    Clone,
    Serialize,
    Deserialize
)]

pub enum MessageType{

    SessionStart,

    SessionMeta,

    ChunkPacket,

    Heartbeat,

    RuntimeError,

    SessionEnd
}


impl EnvelopeV1{

    pub fn validate(

        &self

    )->Result<(),String>{

        if self.version!=PROTOCOL_VERSION{

            return Err(

                format!(
                    "invalid version {}",
                    self.version
                )

            );

        }

        if self
            .session_id
            .trim()
            .is_empty(){

            return Err(

                "missing session_id"
                .into()

            );

        }

        Ok(())

    }

}
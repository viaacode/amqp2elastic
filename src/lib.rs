use std::str;

use serde::{Serialize, Deserialize};
use serde_json;
use serde_xml_rs;
use xmltree::Element;
use chrono::prelude::*;

#[macro_use]
extern crate log;
extern crate serde_derive;


#[derive(Deserialize, Debug)]
pub struct Config {
    #[serde(default="default_amqp_up")]
    pub amqp_user: String,
    #[serde(default="default_amqp_up")]
    pub amqp_passwd: String,
    #[serde(default="default_amqp_host")]
    pub amqp_host: String,
    #[serde(default="default_amqp_port")]
    pub amqp_port: String,
    #[serde(default="default_amqp_vhost")]
    pub amqp_vhost: String,
}

fn default_amqp_up() -> String  {
  String::from("admin")
}

fn default_amqp_host() -> String  {
  String::from("localhost")
}

fn default_amqp_port() -> String  {
  String::from("5672")
}

fn default_amqp_vhost() -> String  {
  String::from("")
}


#[derive(Serialize, Deserialize, Debug)]
pub struct EssenceArchivedEvent {
    #[serde(skip_deserializing)]
    event_name: String,
    #[serde(alias = "timestamp")]
    event_timestamp: String,
    #[serde(skip_deserializing)]
    event_handle_timestamp: String,
    //~ event_handle_time: DateTime<Utc>,   // doesn't seem to work, but it should: no compromises
    file: String,
    pid: String,
    md5sum: String,
    #[serde(alias = "s3bucket")]
    s3_bucket: String,
    #[serde(skip_deserializing)]
    event_payload: String,
    #[serde(skip_deserializing)]
    origin: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct EssenceLinkedEvent {
    #[serde(skip_deserializing)]
    event_name: String,
    #[serde(alias = "timestamp")]
    event_timestamp: String,
    #[serde(skip_deserializing)]
    event_handle_timestamp: String,
    file: String,
    #[serde(alias = "mediaId")]
    media_id: String,
    #[serde(skip_deserializing)]
    event_payload: String,
    #[serde(skip_deserializing)]
    origin: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct EssenceUnlinkedEvent {
    #[serde(skip_deserializing)]
    event_name: String,
    #[serde(alias = "timestamp")]
    event_timestamp: String,
    #[serde(skip_deserializing)]
    event_handle_timestamp: String,
    #[serde(alias = "mediaId")]
    media_id: String,
    #[serde(skip_deserializing)]
    event_payload: String,
    #[serde(skip_deserializing)]
    origin: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ObjectDeletedEvent {
    #[serde(skip_deserializing)]
    event_name: String,
    #[serde(alias = "timestamp")]
    event_timestamp: String,
    #[serde(skip_deserializing)]
    event_handle_timestamp: String,
    #[serde(alias = "mediaId")]
    media_id: String,
    #[serde(skip_deserializing)]
    event_payload: String,
    #[serde(skip_deserializing)]
    origin: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct GetMetadataRequest {
    #[serde(skip_deserializing)]
    event_name: String,
    #[serde(alias = "timestamp")]
    event_timestamp: String,
    #[serde(skip_deserializing)]
    event_handle_timestamp: String,
    #[serde(alias = "mediaId")]
    media_id: String,
    #[serde(alias = "correlationId")]
    correlation_id: String,
    #[serde(skip_deserializing)]
    event_payload: String,
    #[serde(skip_deserializing)]
    origin: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct GetMetadataResponse {
    #[serde(skip_deserializing)]
    event_name: String,
    #[serde(alias = "timestamp", default = "default_timestamp")]
    event_timestamp: String,
    #[serde(skip_deserializing)]
    event_handle_timestamp: String,
    #[serde(alias = "correlationId")]
    correlation_id: String,
    // TODO: metadata struct?
    //~ metadata: String,
    #[serde(skip_deserializing)]
    event_payload: String,
    #[serde(skip_deserializing)]
    origin: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct MetadataUpdatedEvent {
    #[serde(skip_deserializing)]
    event_name: String,
    #[serde(alias = "timestamp")]
    event_timestamp: String,
    #[serde(skip_deserializing)]
    event_handle_timestamp: String,
    // TODO: metadata struct?
    //~ metadata: String,
    #[serde(skip_deserializing)]
    event_payload: String,
    #[serde(skip_deserializing)]
    origin: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ClosedOtAvailableEvent {
    #[serde(skip_deserializing)]
    event_name: String,
    #[serde(alias = "timestamp")]
    event_timestamp: String,
    #[serde(skip_deserializing)]
    event_handle_timestamp: String,
    #[serde(skip_deserializing)]
    event_payload: String,
    #[serde(skip_deserializing)]
    origin: String,
    // ? https://stackoverflow.com/questions/46753955/how-to-transform-fields-during-deserialization-using-serde
    //~ #[serde(alias = "mediaId")]
    //~ media_id: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct OpenOtAvailableEvent {
    #[serde(skip_deserializing)]
    event_name: String,
    #[serde(alias = "timestamp")]
    event_timestamp: String,
    #[serde(skip_deserializing)]
    event_handle_timestamp: String,
    #[serde(skip_deserializing)]
    event_payload: String,
    #[serde(skip_deserializing)]
    origin: String,
    // ? https://stackoverflow.com/questions/46753955/how-to-transform-fields-during-deserialization-using-serde
    //~ #[serde(alias = "mediaId")]
    //~ media_id: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct MakeSubtitleAvailableRequest {
    #[serde(skip_deserializing)]
    event_name: String,
    #[serde(alias = "timestamp", default = "default_timestamp")]
    event_timestamp: String,
    #[serde(skip_deserializing)]
    event_handle_timestamp: String,
    #[serde(alias = "correlationId")]
    correlation_id: String,
    #[serde(alias = "id")]
    media_id: String,
    #[serde(alias = "destinationPath")]
    destination_path: String,
    #[serde(alias = "otType")]
    ot_type: String,
    #[serde(skip_deserializing)]
    event_payload: String,
    #[serde(skip_deserializing)]
    origin: String,
}

#[derive(Serialize, Deserialize, Debug)]
enum VrtEvent {
    EssenceArchivedEvent,
    EssenceLinkedEvent,
    EssenceUnlinkedEvent,
    ObjectDeletedEvent,
    GetMetadataRequest,
    GetMetadataResponse,
    MetadataUpdatedEvent,
    ClosedOtAvailableEvent,
    OpenOtAvailableEvent,
    MakeSubtitleAvailableRequest,
}

impl EssenceArchivedEvent {

    pub fn new(xml: Element, body: &str) -> EssenceArchivedEvent {
        // Deserialize XML to struct
        let mut event: EssenceArchivedEvent = serde_xml_rs::from_str(body).unwrap();
        // Add in other properties
        event.event_name = String::from(&xml.name);
        event.event_payload = body.to_string();
        event.event_handle_timestamp = Utc::now().to_rfc3339();
        event.origin = Origin::Meemoo.to_str();
        return event
    }

    pub fn to_json(&self) -> String {
        serde_json::to_string(self).unwrap()
    }

}

impl EssenceLinkedEvent {

    pub fn new(xml: Element, body: &str) -> EssenceLinkedEvent {
        // Deserialize XML to struct
        let mut event: EssenceLinkedEvent = serde_xml_rs::from_str(body).unwrap();
        // Add in other properties
        event.event_name = String::from(&xml.name);
        event.event_payload = body.to_string();
        event.event_handle_timestamp = Utc::now().to_rfc3339();
        event.origin = Origin::Vrt.to_str();
        return event
    }

    pub fn to_json(&self) -> String {
        serde_json::to_string(self).unwrap()
    }

}

impl EssenceUnlinkedEvent {

    pub fn new(xml: Element, body: &str) -> EssenceUnlinkedEvent {
        // Deserialize XML to struct
        let mut event: EssenceUnlinkedEvent = serde_xml_rs::from_str(body).unwrap();
        // Add in other properties
        event.event_name = String::from(&xml.name);
        event.event_payload = body.to_string();
        event.event_handle_timestamp = Utc::now().to_rfc3339();
        event.origin = Origin::Vrt.to_str();
        return event
    }

    pub fn to_json(&self) -> String {
        serde_json::to_string(self).unwrap()
    }

}

impl ObjectDeletedEvent {

    pub fn new(xml: Element, body: &str) -> ObjectDeletedEvent {
        // Deserialize XML to struct
        let mut event: ObjectDeletedEvent = serde_xml_rs::from_str(body).unwrap();
        // Add in other properties
        event.event_name = String::from(&xml.name);
        event.event_payload = body.to_string();
        event.event_handle_timestamp = Utc::now().to_rfc3339();
        event.origin = Origin::Vrt.to_str();
        return event
    }

    pub fn to_json(&self) -> String {
        serde_json::to_string(self).unwrap()
    }

}

impl GetMetadataRequest {

    pub fn new(xml: Element, body: &str) -> GetMetadataRequest {
        // Deserialize XML to struct
        let mut event: GetMetadataRequest = serde_xml_rs::from_str(body).unwrap();
        // Add in other properties
        event.event_name = String::from(&xml.name);
        event.event_payload = body.to_string();
        event.event_handle_timestamp = Utc::now().to_rfc3339();
        event.origin = Origin::Meemoo.to_str();
        return event
    }

    pub fn to_json(&self) -> String {
        serde_json::to_string(self).unwrap()
    }

}

impl GetMetadataResponse {

    pub fn new(xml: Element, body: &str) -> GetMetadataResponse {
        // Deserialize XML to struct
        let mut event: GetMetadataResponse = serde_xml_rs::from_str(body).unwrap();
        // Add in other properties
        event.event_name = String::from(&xml.name);
        event.event_payload = body.to_string();
        event.event_handle_timestamp = Utc::now().to_rfc3339();
        event.origin = Origin::Vrt.to_str();
        return event
    }

    pub fn to_json(&self) -> String {
        serde_json::to_string(self).unwrap()
    }

}

impl MetadataUpdatedEvent {

    pub fn new(xml: Element, body: &str) -> MetadataUpdatedEvent {
        // Deserialize XML to struct
        let mut event: MetadataUpdatedEvent = serde_xml_rs::from_str(body).unwrap();
        // Add in other properties
        event.event_name = String::from(&xml.name);
        event.event_payload = body.to_string();
        event.event_handle_timestamp = Utc::now().to_rfc3339();
        event.origin = Origin::Vrt.to_str();
        return event
    }

    pub fn to_json(&self) -> String {
        serde_json::to_string(self).unwrap()
    }

}

impl ClosedOtAvailableEvent {

    pub fn new(xml: Element, body: &str) -> ClosedOtAvailableEvent {
        // Deserialize XML to struct
        let mut event: ClosedOtAvailableEvent = serde_xml_rs::from_str(body).unwrap();
        // Add in other properties
        event.event_name = String::from(&xml.name);
        event.event_payload = body.to_string();
        event.event_handle_timestamp = Utc::now().to_rfc3339();
        event.origin = Origin::Vrt.to_str();
        return event
    }

    pub fn to_json(&self) -> String {
        serde_json::to_string(self).unwrap()
    }

}

impl OpenOtAvailableEvent {

    pub fn new(xml: Element, body: &str) -> OpenOtAvailableEvent {
        // Deserialize XML to struct
        let mut event: OpenOtAvailableEvent = serde_xml_rs::from_str(body).unwrap();
        // Add in other properties
        event.event_name = String::from(&xml.name);
        event.event_payload = body.to_string();
        event.event_handle_timestamp = Utc::now().to_rfc3339();
        event.origin = Origin::Vrt.to_str();
        return event
    }

    pub fn to_json(&self) -> String {
        serde_json::to_string(self).unwrap()
    }

}

impl MakeSubtitleAvailableRequest {

    pub fn new(xml: Element, body: &str) -> MakeSubtitleAvailableRequest {
        // Deserialize XML to struct
        let mut event: MakeSubtitleAvailableRequest = serde_xml_rs::from_str(body).unwrap();
        // Add in other properties
        event.event_name = String::from(&xml.name);
        event.event_payload = body.to_string();
        event.event_handle_timestamp = Utc::now().to_rfc3339();
        event.origin = Origin::Meemoo.to_str();
        return event
    }

    pub fn to_json(&self) -> String {
        serde_json::to_string(self).unwrap()
    }

}
#[derive(Debug, Serialize, PartialEq)]
pub enum Origin {
    Vrt,
    Meemoo,
}

impl Origin {
    pub fn to_str(&self) -> String {
        match &*self {
            Origin::Vrt => "vrt".to_string(),
            Origin::Meemoo => "meemoo".to_string(),
        }
    }
}

fn default_timestamp() -> String {
    // Issue bij VRT! -> VD-
    warn!("default_timestamp called!");
    Utc::now().to_rfc3339()
}

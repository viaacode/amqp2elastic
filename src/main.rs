use std::str;

use amiquip::{Connection, ConsumerMessage, ConsumerOptions, Exchange, Publish, Result};
use xmltree::{Element, ParseError};
use envy;

use amqp2elastic::*;

#[macro_use]
extern crate log;
extern crate serde_derive;

// Declare some constants
const IN_QUEUE: &'static str = "vrt2elk_events_xml_q";
const OUT_QUEUE: &'static str = "vrt2elk_events_json_q";


fn handle_xml(xml: Element, body: &str) -> Result<String, &'static str> {
    debug!("{:#?}", xml);
    let root_tag = String::from(&xml.name);
    info!("Root tag is: {:#?}", root_tag);
    match root_tag.as_str() {
        "essenceArchivedEvent" => {
            let event = EssenceArchivedEvent::new(xml, body);
            debug!("{:?}", event);
            // Serialize it to a JSON string
            let j = event.to_json();
            Ok(j)
        },
        "essenceLinkedEvent" => {
            let event = EssenceLinkedEvent::new(xml, body);
            debug!("{:?}", event);
            // Serialize it to a JSON string
            let j = event.to_json();
            Ok(j)
        },
        "essenceUnlinkedEvent" => {
            let event = EssenceUnlinkedEvent::new(xml, body);
            debug!("{:?}", event);
            // Serialize it to a JSON string
            let j = event.to_json();
            Ok(j)
        },
        "objectDeletedEvent" => {
            let event = ObjectDeletedEvent::new(xml, body);
            debug!("{:?}", event);
            // Serialize it to a JSON string
            let j = event.to_json();
            Ok(j)
        },
        "getMetadataRequest" => {    
            let event = GetMetadataRequest::new(xml, body);
            debug!("{:?}", event);
            // Serialize it to a JSON string
            let j = event.to_json();
            Ok(j)
        },
        "getMetadataResponse" => {
            let event = GetMetadataResponse::new(xml, body);
            debug!("{:?}", event);
            // Serialize it to a JSON string
            let j = event.to_json();
            Ok(j)
        },
        "metadataUpdatedEvent" => {
            let event = MetadataUpdatedEvent::new(xml, body);
            debug!("{:?}", event);
            // Serialize it to a JSON string
            let j = event.to_json();
            Ok(j)
        },
        "closedOtAvailableEvent" => {
            let event = ClosedOtAvailableEvent::new(xml, body);
            debug!("{:?}", event);
            // Serialize it to a JSON string
            let j = event.to_json();
            Ok(j)
        },
        "openOtAvailableEvent" => {
            let event = OpenOtAvailableEvent::new(xml, body);
            debug!("{:?}", event);
            // Serialize it to a JSON string
            let j = event.to_json();
            Ok(j)
        },
        "makeSubtitleAvailableRequest" => {
            let event = MakeSubtitleAvailableRequest::new(xml, body);
            debug!("{:?}", event);
            // Serialize it to a JSON string
            let j = event.to_json();
            Ok(j)
        },
        "triggerExportRequest" => {
            let event = TriggerExportRequest::new(xml, body);
            debug!("{:?}", event);
            // Serialize it to a JSON string
            let j = event.to_json();
            Ok(j)
        },
        "triggerExportResponse" => {
            let event = TriggerExportResponse::new(xml, body);
            debug!("{:?}", event);
            // Serialize it to a JSON string
            let j = event.to_json();
            Ok(j)
        },
        _ => {
            warn!("Unknown event type: {:#?}", root_tag);
            Err("Unknown event type")
        },
    }
}

fn handle_error(err: ParseError) {
    warn!("Error: {}", err);
    warn!("If a DLX was specified for q:{}, find the message there", IN_QUEUE);
}

fn format_connection_string(config: &Config) -> String {
    format!("amqp://{}:{}@{}:{}/{}",
        config.amqp_user,
        config.amqp_passwd,
        config.amqp_host,
        config.amqp_port,
        config.amqp_vhost)
}

fn main() -> Result<()> {
    // First and foremost, initialize the logger
    env_logger::init();

    // Get our configuration from the environment
    // The necessary environment variables can be found in the `.env` file
    let config = match envy::from_env::<Config>() {
       Ok(config) => config,
       Err(error) => panic!("{:#?}", error)
    };

    // Open connection.
    let connection_string = format_connection_string(&config);
    let mut connection = Connection::insecure_open(&connection_string)?;

    // Open a channel - None says let the library choose the channel ID.
    let channel = connection.open_channel(None)?;

    // Passively declare the in_queue (meaning the queue should already be
    // declared/configured on the broker).
    let in_queue = channel.queue_declare_passive(IN_QUEUE)?;

    // Publish: Get a handle to the direct exchange on our channel.
    let exchange = Exchange::direct(&channel);

    // Start a consumer.
    let consumer = in_queue.consume(ConsumerOptions::default())?;
    info!("Waiting for messages on q:{}/{} on {}.", config.amqp_vhost, IN_QUEUE, config.amqp_host);
    info!("Consumer tag is: {}", consumer.consumer_tag());
    info!("Press Ctrl-C to exit.");

    for (i, message) in consumer.receiver().iter().enumerate() {
        match message {
            ConsumerMessage::Delivery(delivery) => {
                info!("Routing key: {:?}", &delivery.routing_key);
                let body = str::from_utf8(&delivery.body).unwrap();
                debug!("({:>3}) Received [{}]", i, body);
                let xml_result = Element::parse(body.as_bytes());
                match xml_result {
                    Ok(xml_tree) => {
                        let json_event = handle_xml(xml_tree, body);
                        match json_event {
                            Ok(json_event) => {
                                debug!("{:?}", json_event);
                                exchange.publish(Publish::new(json_event.as_bytes(), OUT_QUEUE))?;
                                consumer.ack(delivery)?;
                            },
                            Err(e) => {
                                warn!("{}", e);
                                warn!("If a DLX was specified for q:{}, find the message there", IN_QUEUE);
                                consumer.reject(delivery, false)?;
                            }
                        }
                    },
                    Err(e) => {
                        handle_error(e);
                        consumer.reject(delivery, false)?;
                    },
                }
            }
            other => {
                info!("Consumer ended: {:?}", other);
                break;
            }
        }
    }
    connection.close()
}

// TODO: UNIT TESTS
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
    #[test]
    fn does_it_work() {
        let a: String = Origin::Vrt.to_str();
        let b: &str = "vrt";
        assert_eq!(a.as_str(), b)
    }
}

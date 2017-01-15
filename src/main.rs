extern crate mqtt3;
extern crate netopt;
extern crate mqttc;
extern crate rustc_serialize;

use std::{time, str};
use mqttc::{PubSub, ClientOptions, ReconnectMethod};
use mqtt3::{Message};
use std::sync::Arc;
use rustc_serialize::json::{Json, encode, decode};

#[derive(RustcDecodable, RustcEncodable, Debug)]
pub struct TrafficItem {
    desi: String,
    dir: String,
    oper: String,
    veh: String,
    tst: String,
    tsi: u32,
    spd: f32,
    lat: f32,
    long: f32,
    dl: f32,
    oday: String,
    jrn: String,
    line: String,
    start: String,
    source: String
}

trait PrettyPrint {
    fn pretty_print(&self);
}

impl PrettyPrint for TrafficItem {
    fn pretty_print(&self) {
        println!("tram {:?} at {:?}, {:?}", self.desi, self.lat, self.long);
    }
}

fn format_message(message: Box<Message>) -> TrafficItem {
    // Unwrap data to String
    let data: String = String::from_utf8(Arc::try_unwrap(message.payload).unwrap()).unwrap();
    // Parse data as JSON
    let data_json = Json::from_str(&data).unwrap();
    let data_parsed = data_json.find("VP").unwrap();
    //println!("data_parsed: {:?}", data_parsed);
    // Construct TrafficItem struct from JSON
    let data_final = encode(&data_parsed).unwrap();
    //println!("data_final: {:?}", data_final);
    let data_struct: TrafficItem = decode(&data_final).unwrap();
    //println!("data_struct: {:?}", data_struct);
    data_struct
}

fn handle_message(message: Box<Message>) {
    //println!("{:?}", message.topic.path);
    let traffic_item = format_message(message);
    traffic_item.pretty_print();
}

fn main() {
    let netopt = netopt::NetworkOptions::new();
    let mut opts = ClientOptions::new();
    opts.set_reconnect(ReconnectMethod::ReconnectAfter(time::Duration::from_secs(1)));
    // API SPECS https://digipalvelutehdas.hackpad.com/ep/pad/static/HSL-MQTT-API-draft
    let api = "213.138.147.225:1883";
    // TOPIC FORMAT
    // /hfp/journey/type/id/line/direction/headsign/start_time/next_stop/(geohash_level)/geohash/#
    let topic = "/hfp/journey/+/+/+/+/+/+/+/60;24/19/82/#";

    let mut client = opts.connect(api, netopt)
        .expect("Can't connect to server");

    client.subscribe(topic).unwrap();
    loop {
        match client.await().unwrap() {
            Some(message) => {
                handle_message(message);
            },
            None => {}
        }
    }
}

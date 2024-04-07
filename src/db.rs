use bevy::prelude::*;
use bevy_http_client::*;

use crate::controls::GetDataEvent;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
struct Book {
    pub isbn: String,
    pub title: String,
    pub author: String,
    pub metadata: Option<Metadata>,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
struct Metadata {
    pub avg_review: f32,
    pub tags: Vec<String>,
}

pub struct DataPlugin;

impl Plugin for DataPlugin {
    fn build(&self, app: &mut App) {
       app .add_plugins(HttpClientPlugin)
       .add_systems(Update, send_request.run_if(on_event::<GetDataEvent>()))
       .add_systems(Update, handle_response);
    }
}
fn send_request(mut ev_request: EventWriter<HttpRequest>) {
    let request = HttpClient::new().get("https://rust-sqlx.onrender.com/bookjson/9901").build();
    ev_request.send(request);
}

fn handle_response(mut ev_resp: EventReader<HttpResponse>) {
    for response in ev_resp.read() {
        //println!("response text: {:?}", response.text());
        //println!("response json: {:?}", response.json::<Book>());
        let data = response.json::<Book>().unwrap();
        let tags = data.metadata.unwrap().tags;
        println!("Highscoreers Name: {:?}", tags[0]);
        println!("Highscoreers Score: {:?}", tags[1]);
    }
}
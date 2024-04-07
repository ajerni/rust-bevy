use bevy::prelude::*;
use bevy_http_client::*;

use crate::{controls::GetDataEvent, Scoreboard, UpdateDataEvent};
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
        app.add_plugins(HttpClientPlugin)
            .add_systems(
                Update,
                send_request_get_highscore.run_if(on_event::<GetDataEvent>()),
            )
            .add_systems(
                Update,
                send_request_update_highscore.run_if(on_event::<UpdateDataEvent>()),
            )
            .add_systems(Update, handle_response);
    }
}
fn send_request_get_highscore(mut ev_request: EventWriter<HttpRequest>) {
    let request = HttpClient::new()
        .get("https://rust-sqlx.onrender.com/bookjson/9901")
        .build();
    ev_request.send(request);
}

fn send_request_update_highscore(
    mut ev_request: EventWriter<HttpRequest>,
    scoreboard: ResMut<Scoreboard>,
) {
    let new_name = &scoreboard.player_name;
    let new_score = &scoreboard.score.to_string();

    let payload = serde_json::json!({"isbn":"9901","title":"Bevy Data","author":"Andi Tester","metadata":{"avg_review":5.0,"tags":[new_name, new_score]}});

    let request = HttpClient::new()
        .patch("https://rust-sqlx.onrender.com/update/9901")
        .json(&payload)
        .build();

    ev_request.send(request);
}

// fn send_request_update_highscore(mut ev_request: EventWriter<HttpRequest>) {
//     let payload = serde_json::json!({"isbn":"9901","title":"Bevy Data","author":"Andi Tester","metadata":{"avg_review":5.0,"tags":["DB still works","1"]}});

//     let request = HttpClient::new()
//         .patch("https://rust-sqlx.onrender.com/update/9901")
//         .json(&payload)
//         .build();

//     ev_request.send(request);
// }


fn handle_response(mut ev_resp: EventReader<HttpResponse>, mut scoreboard: ResMut<Scoreboard>) {
    for response in ev_resp.read() {

        //response text: Some("Book updated successfully.") comes from DB in case of successful update
        println!("response text: {:?}", response.text());

        let data = response.json::<Book>();

        match data {
            Ok(data) => {
                println!("data: {:?}", data);
                let tags = data.metadata.unwrap().tags;
                println!("Highscoreers Name: {:?}", tags[0]);
                scoreboard.highscore_holder = tags[0].to_string();
                println!("Highscoreers Score: {:?}", tags[1]);
                let number_string = &tags[1];
                let highscore: usize = number_string.parse().unwrap();
                scoreboard.highscore = highscore;
            }
            Err(err) => println!("err: {}", err),
        }
    }
}

mod error;

use crate::error::ServiceError;
use actix_web::{web, App, HttpResponse, HttpServer, Responder};
use battito_lib::interpreter::{interpret, RunConfig};
use battito_lib::max::Payload;
use battito_lib::SUBDIVISION_DEFAULT;
use nannou_osc as osc;
use nannou_osc::rosc::OscMessage;
use nannou_osc::{Connected, Sender};
use serde::Deserialize;

struct AppState {
    sender: Sender<Connected>,
    run_config: RunConfig,
}

pub struct Config {
    host: String,
    port: i32,
}

impl Config {
    fn address(&self) -> String {
        format!("{}:{}", self.host, self.port)
    }

    pub fn sender(&self) -> Sender<Connected> {
        osc::sender()
            .expect("Could not bind to default socket")
            .connect(&self.address())
            .expect("Could not connect to socket at address")
    }
}

#[derive(Deserialize)]
struct Info {
    target: String,
    pattern: String,
}

impl Info {
    fn to_parser(&self) -> String {
        format!("{} > {}", self.target, self.pattern)
    }
}

async fn parse(info: web::Json<Info>, data: web::Data<AppState>) -> Result<HttpResponse, ServiceError> {
    let payload = interpret(&info.0.to_parser(), &data.run_config)?;
    let packet = to_osc_message(&payload)?;

    let _ = data.sender.send(packet).map_err(ServiceError::from)?;
    Ok(HttpResponse::Ok().finish())
}

async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        let config = Config {
            host: "127.0.0.1".to_string(),
            port: 1234,
        };
        let run_config = RunConfig {
            subdivision: SUBDIVISION_DEFAULT,
        };
        let json_config = web::JsonConfig::default().limit(4096);
        App::new()
            .data(AppState {
                sender: config.sender(),
                run_config,
            })
            .service(
                web::resource("/parse")
                    .app_data(json_config)
                    .route(web::post().to(parse)),
            )
            .route("/", web::get().to(hello))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}

fn to_osc_message(payload: &Payload) -> Result<OscMessage, ServiceError> {
    Ok(OscMessage {
        addr: serde_json::to_string(payload)?,
        args: None,
    })
}

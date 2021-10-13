use super::config::ConfigurationFile;
use super::quote;

use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use std::{fs, path};

use serde::Deserialize;
use rand::prelude::SliceRandom;
use hyper::service::{make_service_fn, service_fn};
use hyper::{Body, Method, Request, Response, Server, StatusCode};
#[allow(unused_imports)]
use log::{trace, info, error};

#[derive(Debug, Deserialize)]
struct SerialToId {
    serial: u32,
}

#[derive(Debug, Deserialize)]
struct IdToSerial {
    id: u32,
    serial: u32,
}

/// Starts the server that is *the server*, for updates
/// and syncronisation code. This does not scan for lead nodes,
/// as that happens before this function should be run.
pub async fn server(config: ConfigurationFile) {
    let server_config = config.server.unwrap();
    if ! path::Path::new(&server_config.s2id).exists() {
        match fs::File::create(&server_config.s2id) {
            Ok(p) => p,
            Err(e) => { error!("Failed to create persistant s2id database. {}", e); return; }
        };

        // NOTE: Shouldn't ever fail
        fs::write(&server_config.s2id, "{}").unwrap();
    }

    let initial_db: HashMap<u32, u32> = match serde_json::from_str(match &fs::read_to_string(&server_config.s2id) {
        Ok(p) => p,
        Err(e) => { error!("Failed to read from persistant s2id database. {}", e); return; }
    }) {
        Ok(p) => p,
        Err(e) => { error!("Failed to parse JSON in the persistant s2id database. {}", e); return; }
    };

    let serial_to_id: Arc<Mutex<HashMap<u32, u32>>> = Arc::from(Mutex::from(initial_db));
    let bind_on = match (config.host + ":" + &config.port.to_string()).parse() {
        Ok(a) => a,
        Err(e) => { error!("Failed to parse socket address. {}", e); return; }
    };

    let service = make_service_fn(move |_| {
        let db = serial_to_id.clone();
        let s2id = server_config.s2id.clone();

        async move {
            // async block is only executed once, so just pass it on to the closure
            Ok::<_, hyper::Error>(service_fn(move |req| {
                // Cloning DB here is not nearly as memory hungry as it seems;
                // it's an Arc, so we don't have to worry.
                let db = db.clone();
                let s2id = s2id.clone();

                async move { client_dispatch(req, db, s2id).await }
            }))
        }
    });

    let server = match Server::try_bind(&bind_on) {
        Ok(s) => s,
        Err(e) => { error!("Failed to bind on {}. {}", &bind_on, e); return; }
    }.serve(service);
    info!("Listening on http://{}", bind_on);

    match server.await {
        Ok(()) => (),
        Err(e) => {
            error!(
                "The server (Hyper) encountered a error, mistake, misconception, delusion, inaccuracy, miscalculation, blunder, fault, flaw, oversight, or misprint. {}", e);
            return; },
    };
}

async fn client_dispatch(req: Request<Body>, db: Arc<Mutex<HashMap<u32, u32>>>, persistant: String) -> Result<Response<Body>, hyper::Error> {
    let mut db = db.lock().unwrap();

    let ret = match (req.method(), req.uri().path()) {
        (&Method::GET, "/") => Ok(Response::new(Body::from(
            *quote::QUOTES.choose(&mut rand::thread_rng()).unwrap(),
        ))),

        (&Method::GET, "/s2id") => {
            let uri: SerialToId = match serde_qs::from_str(req.uri().query().unwrap_or("")) {
                Ok(u) => u,
                Err(e) => {
                    error!("Failed to parse query string. {}", e);
                    return Ok(Response::builder().status(StatusCode::BAD_REQUEST).body(Body::from("Bad request")).unwrap());
                }
            };

            // Do we actually have that s2id on record?
            // There must be a response for insuffient data, but I can't find it.
            // But what do I know? I'm *just a teapot*.
            if ! db.contains_key(&uri.serial) { return Ok(Response::builder().status(StatusCode::IM_A_TEAPOT).body(Body::from("Data not available")).unwrap()) }
            Ok(Response::new(Body::from(format!(r#"{{"id": {}}}"#, db.get(&uri.serial).unwrap()))))
        },

        (&Method::POST, "/s2id") => {
            let uri: IdToSerial = match serde_qs::from_str(req.uri().query().unwrap_or("")) {
                Ok(u) => u,
                Err(e) => {
                    error!("Failed to parse query string. {}", e);
                    return Ok(Response::builder().status(StatusCode::BAD_REQUEST).body(Body::from("Bad request")).unwrap());
                }
            };

            // Again, check to see if we already have it on record
            let already = db.contains_key(&uri.serial);

            db.insert(uri.serial, uri.id);
            if already {
                return Ok(Response::builder().status(StatusCode::ALREADY_REPORTED).body(Body::from("Updated")).unwrap());
            }

            Ok(Response::builder().status(StatusCode::OK).body(Body::empty()).unwrap())
        }

        // Return the 404 Not Found for other routes.
        _ => {
            let mut not_found = Response::default();
            *not_found.status_mut() = StatusCode::NOT_FOUND;
            Ok(not_found)
        },
    };

    let mut asd = HashMap::new();
    db.clone().into_iter().for_each(|(k, v)| { asd.insert(k, v); });

    match fs::write(persistant, serde_json::to_string(&asd).unwrap()) {
        Ok(()) => (),
        Err(e) => {
            error!("Failed to write to persistant database. NOT ADDING TO TEMPORARY. {}", e);
            return Ok(Response::builder().status(StatusCode::INTERNAL_SERVER_ERROR).body(Body::from("Failed to write to persistant database.")).unwrap());
        }
    };

    ret
}


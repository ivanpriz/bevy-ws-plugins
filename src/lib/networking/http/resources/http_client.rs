use async_channel::{bounded, Receiver, Sender, TryRecvError};

use std::collections::HashMap;
use std::thread;

use bevy::prelude::*;

use crate::networking::http::messages::{GETData, HTTPRequest, HTTPResponse, POSTData};

#[derive(Resource)]
pub struct HTTPClient {
    http_send_sender: Sender<HTTPRequest>,
    http_send_receiver: Receiver<HTTPRequest>,

    http_recv_receiver: Receiver<HTTPResponse>,
    http_recv_sender: Sender<HTTPResponse>,

    tokio_thread: Option<thread::JoinHandle<()>>,
}

impl HTTPClient {
    pub fn new() -> Self {
        let (http_recv_sender, http_recv_receiver) = bounded::<HTTPResponse>(42);
        let (http_send_sender, http_send_receiver) = bounded::<HTTPRequest>(42);

        Self {
            http_send_sender,
            http_send_receiver,
            http_recv_sender,
            http_recv_receiver,
            tokio_thread: None,
        }
    }

    pub fn start(&mut self) {
        let runtime = tokio::runtime::Builder::new_current_thread()
            .enable_all()
            .build()
            .unwrap();

        let http_send_receiver = self.http_send_receiver.clone();
        let http_recv_sender = self.http_recv_sender.clone();

        self.tokio_thread = Some(thread::spawn(move || {
            runtime.block_on(async move {
                let jh = tokio::spawn(async move {
                    let client = reqwest::Client::new();
                    println!(
                        "Http client initiated, entering loop for sending/receiveing requests"
                    );
                    loop {
                        let msg = http_send_receiver
                            .recv()
                            .await
                            .expect("Could not recv message from channel for http requests");

                        // should override
                        let mut req_id = String::new();
                        let resp = match msg {
                            // !TODO: pass headers
                            HTTPRequest::GET(GETData {
                                req_id: request_id,
                                url,
                                not_req_id_headers: headers,
                            }) => {
                                req_id = request_id.clone();
                                client
                                    .get(url)
                                    .headers(
                                        reqwest::header::HeaderMap::try_from(&headers)
                                            .expect("Couldn't parse headers map"),
                                    )
                                    .header(String::from("request_id"), request_id.clone())
                                    .send()
                                    .await
                                    .expect("Failed to send get request")
                            }
                            HTTPRequest::POST(POSTData {
                                req_id: request_id,
                                url,
                                not_req_id_headers: headers,
                                body,
                            }) => {
                                req_id = request_id.clone();
                                client
                                    .post(url)
                                    .headers(
                                        reqwest::header::HeaderMap::try_from(&headers)
                                            .expect("Couldn't parse headers map"),
                                    )
                                    .header(String::from("request_id"), request_id)
                                    .json(&body)
                                    .send()
                                    .await
                                    .expect("Failed to send post request")
                            }
                        };
                        let all_headers = resp
                            .headers()
                            .iter()
                            .map(|(header_name, header_val)| {
                                (
                                    header_name.to_string(),
                                    header_val
                                        .to_str()
                                        .expect("Couldn't parse header value to str")
                                        .to_owned(),
                                )
                            })
                            .collect::<HashMap<String, String>>();

                        let status = resp.status().as_u16();

                        let t = resp.text().await.expect("Couldn't parse to string wtf");
                        let mut fake_body = HashMap::new();
                        // TODO tmp. We actually wanna handle empty bodies as well as json
                        fake_body.insert(String::from("username"), t);

                        http_recv_sender
                            .send(HTTPResponse {
                                request_id: req_id,
                                /*resp
                                .headers()
                                .get("request_id")
                                .expect(
                                    "Received response without request_id header in response",
                                )
                                .to_str()
                                .expect(
                                    "Couldn't parse string from request_id header in response",
                                )
                                .to_owned(),*/
                                all_headers: all_headers,
                                status_code: status,
                                body: fake_body,
                                /*resp
                                .json::<HashMap<String, String>>()
                                .await
                                .expect("Could not convert response body to text"),*/
                            })
                            .await
                            .expect("Could not send http response to channel");
                    }
                });
                println!("Tokio task for http requests sending spawned");
                tokio::try_join!(async move { jh.await }).expect("HTTP tokio task exited");
            })
        }));
    }

    pub fn send_http_request(&mut self, msg: HTTPRequest) {
        self.http_send_sender
            .try_send(msg)
            .expect("Can't send http to internal channel");
    }

    pub fn get_received_http_response(&mut self) -> Option<HTTPResponse> {
        match self.http_recv_receiver.try_recv() {
            Ok(r) => Some(r),
            Err(TryRecvError::Empty) => None,
            Err(TryRecvError::Closed) => panic!("channel closed omg"),
        }
    }
}

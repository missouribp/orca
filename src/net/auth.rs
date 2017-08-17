use std::io::Read;
use std::collections::HashMap;

use http::{Request,  RequestBuilder, Method, Url};
use http::header::{UserAgent};
use json;
use json::Value;

use net::Connection;


/// Contains data for each possible oauth type
#[derive(Debug, Clone)]
pub enum OauthApp {
	/// Not Implemented
	WebApp,
	/// ^
	InstalledApp,
	/// Where args are (app id, app secret) is the script secret
	Script(String, String)
}

#[derive(Debug, Clone)]
pub struct Auth {
	pub app: OauthApp,
	pub username: String,
	pub password: String, // TODO not pub
	pub token: String,
}

#[derive(Debug, Clone)]
pub enum AuthError<'a> {
	UrlError(&'a str),
	ConnectionError(&'a str),
	RequestError(&'a str),
	ResponseError(&'a str)
}

impl Auth {
	fn get_token<'a>(conn: &Connection, app: &OauthApp, username: &String, password: &String) -> Result<String, AuthError<'a>> {
		use self::OauthApp::*;
		match app {
			&Script(ref id, ref secret) => {
				if let Ok(mut tokenreq) = conn.client.post("https://www.reddit.com/api/v1/access_token") {
					let mut params: HashMap<&str, &str> = HashMap::new();
					params.insert("grant_type", "password");
					params.insert("username", &username);
					params.insert("password", &password);
					let tokenreq = tokenreq
							.header(conn.useragent.clone())
							.basic_auth(id.clone(), Some(secret.clone()))
							.form(&params).unwrap();
					if let Ok(mut tokenresponse) = tokenreq.send() {
						if tokenresponse.status().is_success() {
							let mut response = String::new();
							tokenresponse.read_to_string(&mut response).unwrap();
							let responsejson: Value = json::from_str(&response).expect("Got response in unknown format");
							if let Some(token) = responsejson.get("access_token") {
								let token = token.as_str().unwrap().to_string();
								Ok(token)
							} else {
								Err(AuthError::ResponseError("Couldn't parse response from reddit"))
							}
						} else {
							eprintln!("{:?}", tokenresponse);
							Err(AuthError::RequestError("Reddit failed to process the request"))
						}
					} else {
						Err(AuthError::ConnectionError("Couldn't connect to reddit"))
					}
				} else {
					Err(AuthError::UrlError("Badly formed url"))
				}
			},
			_ => panic!("App type not implemented")
		}
	}
	
	pub fn new(conn: &Connection, app: OauthApp, username: String, password: String) -> Result<Auth, AuthError> {
		match Auth::get_token(conn, &app, &username, &password) {
			Ok(token) => {
				Ok(Auth {
					app,
					username,
					password,
					token
				})
			},
			Err(err) => Err(err)
		}
	}
}
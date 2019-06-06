use crate::server::{helper, HttpMessage, HttpResponse, PayloadError, Request};
use failure::Error;
use reqwest::StatusCode;
use test_server as server;

#[test]
fn start_server_at_given_port() -> Result<(), Error> {
    let server = server::new(65432, |_| HttpResponse::Ok().into())?;

    assert!(&server.url().contains(":65432"));

    let response = reqwest::get(&server.url()).unwrap();

    assert_eq!(StatusCode::OK, response.status());

    Ok(())
}

#[test]
#[cfg(not(target_os = "windows"))] // carllerche/mio#776
fn restart_server_at_same_port() -> Result<(), Error> {
    let mut server = server::new(65433, |_| HttpResponse::Ok().into())?;
    let response = reqwest::get(&server.url()).unwrap();

    assert_eq!(StatusCode::OK, response.status());

    server.stop();
    server = server::new(65433, |_| HttpResponse::BadRequest().into())?;
    let response = reqwest::get(&server.url()).unwrap();

    assert_eq!(StatusCode::BAD_REQUEST, response.status());

    Ok(())
}

#[test]
fn validate_client_request() -> Result<(), Error> {
    let server = server::new(0, |req| {
        HttpResponse::Ok().streaming(req.take_payload().into())
    })?;

    let request_content = helper::random_string(100);
    let client = reqwest::Client::new();
    let _ = client
        .post(&server.url())
        .body(request_content.clone())
        .send();

    assert_eq!(server.requests.len(), 1);

    let request = server.requests.next();
    assert!(request.is_some());

    let Request {
        ref body,
        ref headers,
        ref method,
        ref path,
        ref query,
    } = request.unwrap();

    assert_eq!(&request_content, body);
    assert_eq!(Some(&String::from("100")), headers.get("content-length"));
    assert_eq!("POST", method);
    assert_eq!("/", path);
    assert!(query.is_empty());

    Ok(())
}

#[test]
fn not_necessary_to_fetch_request_from_server() -> Result<(), Error> {
    let server = server::new(0, |_| {
        let content = helper::read_file("tests/sample.json").unwrap();
        HttpResponse::Ok().body(content)
    })?;
    let mut response = reqwest::get(&server.url()).unwrap();

    assert_eq!(
        helper::read_file("tests/sample.json").unwrap(),
        response.text().unwrap()
    );

    Ok(())
}

#[test]
fn fetch_2nd_request_from_server() -> Result<(), Error> {
    let server = server::new(0, |_| HttpResponse::Ok().into())?;

    let _ = reqwest::get(&server.url()).unwrap();
    let _ = reqwest::Client::new().post(&server.url()).body("2").send();

    assert_eq!(server.requests.len(), 2);

    let _ = server.requests.next();
    let request = server.requests.next();

    assert!(request.is_some());
    assert_eq!("2", request.unwrap().body);

    Ok(())
}

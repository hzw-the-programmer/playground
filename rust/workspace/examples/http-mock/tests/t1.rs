use httpmock::prelude::*;

#[test]
fn getting_started_test() {
    let server = MockServer::start();

    let mock = server.mock(|when, then| {
        when.method("GET")
            .path("/translate")
            .query_param("word", "hello");

        then.status(200)
            .header("content-type", "text/html; charset=UTF-8")
            .body("hola");
    });

    let resp = reqwest::blocking::get(server.url("/translate?word=hello")).unwrap();

    mock.assert();

    assert_eq!(resp.status(), 200);
}

#[tokio::test]
async fn async_getting_started_test() {
    let server = MockServer::start_async().await;

    let mock = server
        .mock_async(|when, then| {
            when.method(GET)
                .path("/translate")
                .query_param("word", "hello");

            then.status(200)
                .header("content-type", "text/html; charset=UTF-8")
                .body("hola");
        })
        .await;

    let client = reqwest::Client::new();
    let resp = client
        .get(server.url("/translate?word=hello"))
        .send()
        .await
        .unwrap();

    mock.assert();

    assert_eq!(resp.status(), 200);
}

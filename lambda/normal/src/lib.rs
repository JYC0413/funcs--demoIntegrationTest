use lambda_flows::{request_received, send_response};
use flowsnet_platform_sdk::logger;

#[no_mangle]
pub fn run() {
    logger::init();
    request_received(|_qry, _body| {
        log::debug!("mmmmm");
        log::error!("mmmmm");
        send_response(
            200,
            vec![(String::from("content-type"), String::from("text/html"))],
            "ok".as_bytes().to_vec(),
        );
    });
}

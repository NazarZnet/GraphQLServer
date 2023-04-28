mod api;
mod app;
mod errors;
mod schemas;


use std::net::TcpListener;

use app::create_server;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("debug"));

    let port = dotenv::var("PORT")
        .expect("Set PORT in .env file")
        .parse::<u16>()
        .expect("PORT must be a number");
    let url = dotenv::var("URL").expect("Set URL in .env file");
    let address=format!("{}:{}",url,port);
    let listener=TcpListener::bind(address).expect("Can not create address");
    create_server(listener)?.await
}

#[cfg(test)]
mod tests{
    use std::net::TcpListener;

    use crate::app::create_server;

    #[actix_web::test]
    async fn server_health_check(){
        let listener=TcpListener::bind("127.0.0.1:0").expect("Can not create address");
        let addr=listener.local_addr().unwrap().to_string();

        let _server=tokio::spawn(create_server(listener).expect("Can not run server!"));

        let res=awc::Client::default().get(format!("http://{}/",addr)).send().await.expect("Can not send request");
        assert!(res.status().is_success());
    }
}
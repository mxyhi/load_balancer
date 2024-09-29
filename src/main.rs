use async_trait::async_trait;
use log::info;
use pingora::{
    http::RequestHeader,
    lb::{health_check, LoadBalancer},
    listeners::TlsSettings,
    prelude::{background_service, HttpPeer, Opt, RoundRobin},
    proxy::{http_proxy_service, ProxyHttp, Session},
    server::Server,
    Result,
};
use std::{env, sync::Arc, time::Duration};

pub struct LB(Arc<LoadBalancer<RoundRobin>>);

#[async_trait]
impl ProxyHttp for LB {
    type CTX = ();
    fn new_ctx(&self) -> Self::CTX {
        ()
    }

    async fn upstream_peer(&self, _session: &mut Session, _ctx: &mut ()) -> Result<Box<HttpPeer>> {
        let upstream = self
            .0
            .select(b"", 256) // hash doesn't matter
            .unwrap();

        info!("upstream peer is: {:?}", upstream);

        let sni = std::env::var("SNI_DOMAIN");

        if sni.is_ok() {
            let peer = Box::new(HttpPeer::new(upstream, true, sni.unwrap()));
            return Ok(peer);
        }

        let peer = Box::new(HttpPeer::new(upstream, false, "".to_string()));
        Ok(peer)
    }

    async fn upstream_request_filter(
        &self,
        _session: &mut Session,
        upstream_request: &mut RequestHeader,
        _ctx: &mut Self::CTX,
    ) -> Result<()> {
        let host_var = std::env::var("HEADER_HOST");
        if host_var.is_ok() {
            let host_var = host_var.unwrap();
            upstream_request.insert_header("Host", host_var).unwrap();
        }
        Ok(())
    }
}

fn main() {
    env_logger::init();

    let opt = Opt::parse_args();

    let mut my_server = Server::new(Some(opt)).unwrap();
    my_server.bootstrap();

    let upstreams_string = env::var("UPSTREAMS").expect("env UPSTREAMS not set");

    info!("UPSTREAMS: {:?}", upstreams_string);

    let upstreams_address: Vec<String> =
        upstreams_string.split(',').map(|s| s.to_string()).collect();

    let mut upstreams = LoadBalancer::try_from_iter(upstreams_address).unwrap();

    let hc = health_check::TcpHealthCheck::new();
    upstreams.set_health_check(hc);
    upstreams.health_check_frequency = Some(Duration::from_secs(1));

    let background = background_service("health check", upstreams);

    let upstreams = background.task();

    let mut lb = http_proxy_service(&my_server.configuration, LB(upstreams));

    let h1_addr = env::var("H1_ADDR").expect("env H1_ADDR not set");

    lb.add_tcp(&h1_addr);

    info!("h1 server address: {}", h1_addr);

    let h2_addr = std::env::var("H2_ADDR");

    if h2_addr.is_ok() {
        let h2_addr = h2_addr.unwrap();
        let cert_path = env::var("H2_CERT_PATH").expect("env H2_CERT_PATH not set");
        let key_path = env::var("H2_KEY_PATH").expect("env H2_KEY_PATH not set");

        let mut tls_settings = TlsSettings::intermediate(&cert_path, &key_path).unwrap();
        tls_settings.enable_h2();
        lb.add_tls_with_settings(&h2_addr, None, tls_settings);

        info!("h2 server address: {}", h2_addr);
    }

    my_server.add_service(lb);
    my_server.add_service(background);

    info!(
        "starting server threads: {}",
        my_server.configuration.threads
    );

    my_server.run_forever();
}

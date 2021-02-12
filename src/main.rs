use futures::stream::StreamExt;
fn main() {
    tracing_subscriber::fmt::init();

    let runtime = tokio::runtime::Builder::new_multi_thread()
        .enable_all()
        .build()
        .unwrap();
    let client: hyper::Client<hyperlocal::UnixConnector> = hyper::Client::builder()
        // .pool_max_idle_per_host(0) // workaround
        .build(hyperlocal::UnixConnector);
    for i in 0.. {
        println!("{}", i);
        let res = runtime
            .block_on(async {
                let _resp = client
                    .get(hyperlocal::Uri::new("/var/run/docker.sock", "//events?").into())
                    .await;
                client
                    .get(hyperlocal::Uri::new("/var/run/docker.sock", "/events?").into())
                    .await
            })
            .unwrap();
        runtime.spawn(res.into_body().into_future());
    }
}

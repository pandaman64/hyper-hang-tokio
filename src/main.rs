use futures::prelude::*;
fn main() {
    // env_logger::Builder::from_default_env()
    //     .target(env_logger::fmt::Target::Stdout)
    //     .init();
    tracing_subscriber::fmt::init();

    let mut runtime = tokio::runtime::Builder::new()
        .threaded_scheduler()
        .enable_all()
        .build()
        .unwrap();
    let client: hyper::Client<hyperlocal::UnixConnector> =
        hyper::Client::builder().build(hyperlocal::UnixConnector);
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

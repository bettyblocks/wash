use std::{collections::HashMap, io::Read};

use anyhow::Context;
use gag::BufferRedirect;

use wash_runtime::{
    engine::Engine,
    host::{HostApi, HostBuilder},
    types::{Component, Service, Workload, WorkloadStartRequest},
};

const CRON_SERVICE_WASM: &[u8] =
    include_bytes!("../../cron-service/target/wasm32-wasip2/release/cron-service.wasm");

const CRON_COMPONENT_WASM: &[u8] =
    include_bytes!("../../cron-component/target/wasm32-wasip2/release/cron_component.wasm");

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    //let mut stderr_capture = BufferRedirect::stderr().expect("failed to redirect stderr");

    println!("Starting cron-service integration test");

    // Create engine
    let engine = Engine::builder().build()?;

    // Build host with no plugins
    let host = HostBuilder::new().with_engine(engine.clone()).build()?;

    println!("Created host with no plugins");

    // Start the host
    let host = host.start().await.context("Failed to start host")?;
    println!("Host started");

    // Create a workload request with a service and component
    let req = WorkloadStartRequest {
        workload: Workload {
            namespace: "test".to_string(),
            name: "cron-service-workload".to_string(),
            annotations: HashMap::new(),
            service: Some(Service {
                bytes: bytes::Bytes::from_static(CRON_SERVICE_WASM),
                local_resources: Default::default(),
                max_restarts: 0,
            }),
            components: vec![Component {
                bytes: bytes::Bytes::from_static(CRON_COMPONENT_WASM),
                local_resources: Default::default(),
                max_invocations: 1,
                pool_size: 0,
            }],
            host_interfaces: vec![],
            volumes: vec![],
        },
    };

    // Start the workload
    let _workload_response = host
        .workload_start(req)
        .await
        .context("Failed to start cron-service workload")?;

    println!("Workload started successfully");
    println!("Waiting for service to execute (5 seconds)...");

    dbg!(_workload_response);

    tokio::signal::ctrl_c().await?;

    Ok(())
}

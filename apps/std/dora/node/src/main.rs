use dora_node_api::Event;
use dora_node_api::{self, arrow::array::UInt64Array, dora_core::config::DataId, DoraNode};
use rand::Rng;
// use std::time::Duration;
use uhlc::system_time_clock;

#[no_mangle]
pub extern "C" fn ceil() {
    println!("ceil");
}

#[no_mangle]
pub extern "C" fn sqrt() {
    println!("sqrt");
}

fn main() -> eyre::Result<()> {
    ceil();
    sqrt();
    let latency = DataId::from("latency".to_owned());
    let _throughput = DataId::from("throughput".to_owned());

    let (mut node, mut events) = DoraNode::init_from_env()?;
    // let (mut node, mut events) = DoraNode::init_from_file("node.yml")?;
    let sizes = [1, 10 * 512, 100 * 512, 1000 * 512, 10000 * 512];

    // test latency first
    for size in sizes {
        for i in 0..100 {
            if let Some(event) = events.recv() {
                println!("node recv event[{}] {:#?}", i, event);
                match event {
                    Event::Input {
                        id: _,
                        data: _,
                        metadata,
                    } => {
                        let mut random_data: Vec<u64> = rand::thread_rng()
                            .sample_iter(rand::distributions::Standard)
                            .take(size)
                            .collect();
                        let t_send = system_time_clock().as_u64();
                        let beginning_slice = random_data.get_mut(0).unwrap();
                        *beginning_slice = t_send;

                        let random_data: UInt64Array = random_data.into();

                        println!("node send_output random_data[{}] {:#?}", i, random_data);

                        node.send_output(latency.clone(), metadata.parameters, random_data)?;
                    }
                    _ => {}
                }
            } else {
                break;
            }
        }
    }

    Ok(())
}
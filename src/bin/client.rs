use lazy_static::lazy_static;
use std::net::SocketAddr;
use volo_gen::volo::example::GetItemRequest;
use volo_example::FilterLayer;
use volo_gen::volo::example::RedisCommand;
use pilota::FastStr;

lazy_static! {
    static ref CLIENT: volo_gen::volo::example::ItemServiceClient = {
        let addr: SocketAddr = "127.0.0.1:8080".parse().unwrap();
        volo_gen::volo::example::ItemServiceClientBuilder::new("volo-example")
            .layer_outer(FilterLayer)
            .address(addr)
            .build()
    };
}

#[volo::main]
async fn main() {
    tracing_subscriber::fmt::init();
    let args: Vec<String> = std::env::args().collect();
    let mut req = GetItemRequest { cmd: RedisCommand::Get, key: None, value: None };
    let cmd = args[1].to_lowercase();
    if cmd == String::from("get") {
        req = GetItemRequest{
            cmd : RedisCommand::Get,
            key : Some(FastStr::new(&args[2].clone())),
            value : None,
        };
    }
    else if cmd == String::from("set") {
        req = GetItemRequest{
            cmd : RedisCommand::Set,
            key : Some(FastStr::new(&args[2].clone())),
            value : Some(FastStr::new(&args[3].clone())),
        };
    }
    else if cmd == String::from("del") {
        req = GetItemRequest{
            cmd : RedisCommand::Del,
            key : Some(FastStr::new(&args[2].clone())),
            value : None,
        };
    }
    else if cmd == String::from("ping") {
        req = GetItemRequest{
            cmd : RedisCommand::Ping,
            key : None,
            value : None,
        };
    }
    else {
        panic!("ILLEGAL COMMAND!");
    }

    let resp = CLIENT.get_item(req).await;
    match resp {
        Ok(info) => {
            if info.flag {
                println!("{:?} succeeded!", cmd);
                if cmd == String::from("get") {
                    println!("The result is: {:?}", info.res);
                }
            }
            else { 
                println!("{:?} failed!", cmd);
                if cmd == String::from("get") {
                    println!("NotFound!");
                }
            }
        },
        Err(e) => tracing::error!("{:?}", e),
    }
}

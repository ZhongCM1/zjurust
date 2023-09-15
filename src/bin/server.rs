use std::collections::HashMap;
use std::net::SocketAddr;
use std::sync::Mutex;
use std::fs::File;
use std::io::{BufRead, BufReader};
use volo_redis::S;

//use std::net::TcpListener;

#[volo::main]
async fn main() {
    let addr: SocketAddr = "[::]:8080".parse().unwrap();
    let addr = volo::net::Address::from(addr);

    //let s_addr: SocketAddr = "[::]:8081".parse().unwrap();

    // 1. 读取日志文件并进行数据恢复
    let file = File::open("redis.log").expect("Failed to open redis log file.");
    let reader = BufReader::new(file);

    let map = Mutex::new(HashMap::<String, String>::new());

    for line in reader.lines() {
        if let Ok(line) = line {
            let tokens: Vec<&str> = line.split(' ').collect();

            match tokens[0].to_uppercase().as_str() {
                "SET" => {
                    if tokens.len() >= 3 {
                        map.lock().unwrap().insert(tokens[1].to_string(), tokens[2].to_string());
                    } else {
                        println!("Invalid SET command format: {}", line);
                    }
                }
                "DEL" => {
                    if tokens.len() >= 2 {
                        map.lock().unwrap().remove(&tokens[1].to_string());
                    } else {
                        println!("Invalid DEL command format: {}", line);
                    }
                }
                _ => {
                    println!("Unknown command: {}", line);
                }
            }
        }
    }

    //let listener = TcpListener::bind(s_addr).expect("Failed to bind master node address.");

    //处理从节点连接请求
    // for stream in listener.incoming() {
    //     match stream {
    //         Ok(stream) => {
    //             let map_clone = Arc::clone(&map);
    //             tokio::spawn(async move {
    //                 handle_slave_connection(stream, map_clone).await;
    //             });
    //         }
    //         Err(e) => {
    //             println!("Error accepting connection: {}", e);
    //         }
    //     }
    // }

    // 2. 创建Redis服务并启动
    volo_gen::volo::redis::RedisServiceServer::new(S { map })
        .run(addr)
        .await
        .unwrap();

}

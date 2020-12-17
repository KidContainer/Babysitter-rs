use tokio::net::TcpStream;
use tokio::prelude::*;
use ::log::{debug, error, info, logger, trace, warn, LevelFilter};


pub async fn pop_one(socket:&mut TcpStream)->Option<String>{
    let mut buf = [0u8;1024];
    let n = match socket.read(&mut buf).await{
        Ok(n)=>n,
        Err(e)=>{
            error!("something wrong, {}",e);
            0
        }
    };

    

    None
}
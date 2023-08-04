use tokio::{net::TcpListener, io::AsyncReadExt, io::AsyncWriteExt};
use std::net::SocketAddr;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let addr: SocketAddr = "127.0.0.1:2342".parse()?;
    let listener = TcpListener::bind(&addr).await?;

    println!("Moss-Server listening on {}", addr);

    while let Ok((mut stream, _)) = listener.accept().await {
        tokio::spawn(async move {
            handle_connection(&mut stream).await;
        });
    }

    Ok(())
}

async fn handle_connection(stream: &mut tokio::net::TcpStream) {
    let mut buffer = [0; 1024];

    loop {
        match stream.read(&mut buffer).await {
            Ok(n) if n > 0 => {
                let request = String::from_utf8_lossy(&buffer[..n]);
                let response = handle_request(&request);
                if let Err(e) = stream.write_all(response.as_bytes()).await {
                    eprintln!("Error writing response: {}", e);
                }

            }
            Ok(_) => {
                // Connection closed
                break;
            }
            Err(e) => {
                eprintln!("Error reading from stream: {}", e);
                break;
            }
        }
    }
}

fn handle_request(request: &str) -> String {
    // API logic goes here
    
    let data: Vec<&str> = request.trim().split(" ").collect();

    if data.len() != 2 {
        return format!("Error: Invalid length of request\n");
    }
    
    let request_method = data[0];

    match request_method {
        "GET" => {
            let request_data = data[1];

            match request_data {
                "config" => {
                    return "Config file sent! (lol not really tho :p )\n".to_string();
                    // Load config file
                    // Return config file
                }
                e => return format!("Error: Invalid GET request {}\n", e)
            }

        }
        e => return format!("Error: Invalid method {}\n", e),
    }

    // return format!("Recieved: {}\n", request);
}

#[cfg(test)]
mod tests {
    use crate::handle_request;

    #[test]
    fn test_handle_request() {
        assert_eq!(handle_request("TEST"),"Error: Invalid length of request\n");
        assert_eq!(handle_request("TEST/test"),"Error: Invalid length of request\n");
        assert_eq!(handle_request("TEST test test2"),"Error: Invalid length of request\n");
        assert_eq!(handle_request("TEST test"),"Error: Invalid method TEST\n");
        assert_eq!(handle_request("GET test"),"Error: Invalid GET request test\n");
        assert_eq!(handle_request("GET config"),"Config file sent! (lol not really tho :p )\n");
    }
}

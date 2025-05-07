mod cli;



fn main() {

}



// // A basic TCP echo server with Tokio 
// #[tokio::main]
// async fn main() -> Result<(), Box<dyn std::error::Error>> {
//     let listener = TcpListener::bind("127.0.0.1:8080").await?;

//     loop {
//         let (mut socket, _) = listener.accept().await?;
        
//         tokio::spawn(async move {
//             let mut buf = [0; 1024];

//             // read data from the socket and write the data back.
//             loop {
//                 let n = match socket.read(&mut buf).await {
//                     //socket closed 
//                     Ok(0) => return,
//                     Ok(n) => n, 
//                     Err(e) => {
//                         eprintln!("Failed to read from socket: Err = {:?}", e);
//                         return;
//                     }
//                 };

//                 // writing the data back
//                 if let Err(e) = socket.write_all(&buf[0..n]).await {
//                         eprintln!("Failed to read from socket: Err = {:?}", e);
//                         return;
//                 }
//             }
//         });
//     }
// }




//repeating code for any number of inputs using macro_rules!
// macro_rules! say_hello {
//     ($($name:expr),*) => {
//         $( println!("Hello, {}!", $name); )*
//     };
// } q

// say_hello!("Leonard", "Alice", "Joyce");
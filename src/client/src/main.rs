//////////////////////////////////////////////
//                                          //
//                  Client                  //
//                                          //
//////////////////////////////////////////////

use std::collections::hash_map::RandomState;
use std::io::{self, ErrorKind, Read, Write};
use std::net::TcpStream;
use std::sync::mpsc::{self, TryRecvError,Receiver};
use std::thread;
use std::time::{Duration, Instant};
use std::str;

use crossbeam_channel::tick;
use libp2p::PeerId;

use crate::message::*;
use crate::node::Node;
pub mod message;
pub mod message_test;
pub mod tiker_test;
mod simple_db;
pub mod state;
mod node;
mod node_test;
mod json_test;
mod VSSS;
mod VSSS_test;

// Localhost with a port in it
const LOCAL_HOST: &str = "127.0.0.1:8081";

// The buffer size of messages
const MESSAGE_SIZE: usize = 32;
pub struct id{
    id:PeerId
}
impl id{
    pub fn new()->Self{
        Self{
            id: PeerId::random(),
        }
    }
}

fn main() 
{   
    let mut node=Node::new();
    let id = id::new();
    // Create a mutable client which is a TCP stream 
    // Connect it to our local here ==> IP with the port 
    let mut client = TcpStream::connect(LOCAL_HOST).expect("Failed to connect");
    // We want our client to be non-blocking
    // Set the flag non-blocking to true
    client.set_nonblocking(true).expect("Failed to initiate non-blocking"); 
    // Instantiate channel and assign it to a string type
    // We are going to be sending a bunch of strings through channel
    let (sender, receiver) = mpsc::channel::<String>();
    
    // Spawn a thread and create a move closure inside of it with a loop
    thread::spawn(move || 
    {
        handler(receiver,client);
        
        // Have our thread sleep for a hundred milliseconds
        thread::sleep(Duration::from_millis(100));
    }); 
    // This will show up when the user opens the client
    println!("*********************************");
    println!("************ WELCOME ************");
    println!("*********************************");

    loop 
    {   
        let mut message=Message::new();
        message.change_view(1);
        message.change_Preprepare("Preprepare".to_string());
        message.change_prepare("Prepare".to_string());
        message.change_commit("Commit".to_string());
        // Create a new mutable string
        let mut buffer = String::new();
        // Read into that string from our standard input
        //println!("plz inter your state");
        io::stdin().read_line(&mut buffer).expect("Reading from stdin failed");
        let mut buffer_clone=buffer.clone();
        //message.change_Preprepare(buffer);
        // Trim our buffer 
        // Use the to string method to put it into a message variable 
        let m = buffer_clone.trim().to_string();
        if m=="change" {
            node.change_to_main();
            //println!("{:?}",node.node_type)
        }
        match node.node_type {
            node::Node_type::Main=>{println!("Main")},
            node::Node_type::Replica=>{println!("Replica")}
        }
        // If message is equivalent to : exit we'll break out of our loop 
        if m == "exit" || sender.send(message.to_string()).is_err() {break}
    }
    // Print out GOOD BYE
    println!("*********************************");
    println!("*********** GOOD BYE ************");
    println!("*********************************");
}

fn simple_ticker(){
    std::thread::spawn(move || loop{
        let message_alive="alive";
        thread::sleep(Duration::from_secs(10))
    });
}
// fn simple_ticker() {

//     let start = Instant::now();
//     let ticker = tick(Duration::from_millis(100));

//     for _ in 0..5 {
//         let msg = ticker.recv().unwrap();
//         println!("{:?} elapsed: {:?}",msg, start.elapsed());
//     }

// }

pub fn handler(mut receiver: Receiver<String>,mut client: TcpStream){
    loop {
        let mut buffer = vec![0; MESSAGE_SIZE];
        // Read our message through the buffer
        match receiver.try_recv() 
        {
            Ok(message) => 
            {
                // Clone message into bytes
                // Put it inside of a buffer variable
                let mut buffer = message.clone().into_bytes();
                // Resize our buffer by our message size
                buffer.resize(MESSAGE_SIZE, 0);
                // Write all of our buffers into our client
                client.write_all(&buffer).expect("Writing to socket failed");
                // Print out the message
                
            },
            /* 
             * Check if our try receive error is empty and 
             * if it is then we're just going to send back a unit type
             */
            Err(TryRecvError::Empty) => (),
            /*
             * Check if it's a disconnected type 
             * in which case we just want to break the loop
             */
            Err(TryRecvError::Disconnected) => break
        }
        match client.read_exact(&mut buffer) 
        {
            Ok(_) => 
            {
                // Let message equal our buffer 
                // Turn it into an iterator 
                // Check to see if the references inside of it are equal to 0 
                // Collect all of them inside of our vector
                // All the ones that are equal to 0 are going to just discard
                let message = buffer.into_iter().take_while(|&x| x != 0).collect::<Vec<_>>();
                let message = str::from_utf8(&message).unwrap();
                //let message=message.to_string();
                let the_message=Message::get_self(message);
                println!("Message :{:?}", the_message);
            },
            /* 
             * If the type of error is equal to an error that would block our non-blocking,
             * we just send back a unit type. 
             */ 
            Err(ref err) if err.kind() == ErrorKind::WouldBlock => (),
            // If we get an error we don't care about what's inside of it 
            // We just close the connection and then we just break out of this loop
            Err(_) => 
            {
                println!("Connection with server was severed");
                break;
            }
        }
    }
}
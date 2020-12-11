mod packet;

struct MPv1Node {
    mac: u64,                       // Unique ID 
    links: Vec<u32>,                // Physical Connections
    connections: Vec<String>,       // Known Nodes in Network
    memory: Vec<packet::MPv1Packet>,// Packets to be processed
    storage: Vec<String>,           // Data needed for future use  
}
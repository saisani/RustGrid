// Mesh Protocal Version 1 Packet
pub struct MPv1Packet<'a> {
    src: &'a str,
    dst: &'a str,
    hops: u32,
    path: Vec<String>,
    next_hop: u32,
}
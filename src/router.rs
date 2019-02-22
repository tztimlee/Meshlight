
extern crate alloc;
use self::alloc::collections::{BTreeMap, VecDeque};
use cortex_m_semihosting::hprintln;
use self::alloc::vec::Vec;

type NodeID = u8;
type PinID = u8;
type Cost = u8;

// RoutingTable associates every known node with its
// cost (number of network hops to reach), and the pin
// where the message is to be sent initially.
type RoutingTable = BTreeMap<NodeID, RoutingTableEntry>;

struct RoutingTableEntry {
    cost: Cost,
    link_id: PinID
}

// DitsanceVector is a summary of the routing table, without
// node-specific information (like what pins to send data on)
type DistanceVector = BTreeMap<NodeID, Cost>;

// Message is the data passed along to a specific
// node. It is routed depending on the given NodeID.
type Message = (NodeID, MessageBody);

// MessageBody is the actual information given to the
// destination node.
enum MessageBody {
    DistanceVector(DistanceVector),
}

const END_BYTE: u8 = 0xFF;

trait Serialize {
    fn serialize(&self) -> Vec<u8>;
}

impl Serialize for Message {
    fn serialize(&self) -> Vec<u8> {
        let mut data = Vec::new();
        match &self {
            (node_id, MessageBody::DistanceVector(dist_vec)) => {
                data.push(*node_id);
                for (node_id, cost) in dist_vec.iter() {
                    data.push(*node_id);
                    data.push(*cost);
                }
            }
        }
        data.push(END_BYTE);
        data
    }
}

// Router is the routing instance for this node
struct Router {
    id: NodeID,
    routing_table: RoutingTable,
}

impl Router {
    pub fn new(id: NodeID) -> Router {
        Router{
            id: id,
            routing_table: RoutingTable::new()
        } 
    }
    pub fn broadcast_dist_vec(&self) {
        let dist_vec = self.get_distance_vec();
        for (node_id, node) in &self.routing_table {
            send(*node_id, MessageBody::DistanceVector(dist_vec.clone()));
        } 
    }
    pub fn receive_msg(&mut self, pin_id: PinID, msg: Message) {
        let (node_id, msg_body) = msg;
        // Pass message along if not meant for us
        if node_id != self.id {
            send(node_id, msg_body);
            return;
        }
        match msg_body {
            MessageBody::DistanceVector(dist_vec) => {
                self.receive_dist_vec(pin_id, dist_vec);
            },
        }
    }
    pub fn receive_dist_vec(&mut self, pin_id: PinID, dist_vec: DistanceVector) {
        for (node_id, cost) in dist_vec {
            match self.routing_table.get(&node_id) {
                // If seen route before, check if better
                Some(node) => {
                    let new_cost = cost + 1;
                    if new_cost < node.cost {
                        self.add_connection(node_id, pin_id, new_cost);
                    }
                },
                // If new route then save
                None => {
                    self.add_connection(node_id, pin_id, cost) 
                }
            }
        }  
    }
    pub fn add_connection(&mut self, node_id: NodeID, link_id: PinID, cost: Cost) {
        self.routing_table.insert(node_id, RoutingTableEntry{
            cost: cost,
            link_id: link_id
        });
    }
    pub fn get_distance_vec(&self) -> DistanceVector {
        let mut dist_vec = DistanceVector::new();
        for (node_id, node) in &self.routing_table {
            dist_vec.insert(*node_id, node.cost);
        }
        dist_vec
    }
}

fn send(node_id: NodeID, msg_body: MessageBody) {
    let message = (node_id, msg_body);
    let bytes = message.serialize();
    // Connect to USART send message in main.rs
}

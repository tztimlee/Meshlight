
extern crate alloc;
use self::alloc::collections::{BTreeMap, VecDeque};

type Cost = u32;
type PinID = u32;
type NodeID = u32;

type RoutingTable = BTreeMap<NodeID, RoutingTableEntry>;

enum Message {
    DistanceVector(NodeID, DistanceVector),
    Heartbeat(NodeID, NodeID),
    SendMsg(NodeID, u32)
}

use cortex_m_semihosting::hprintln;

struct RoutingTableEntry {
    cost: Cost,
    neighbour_id: PinID
}

// DistanceVector is a summary of the RoutingTable
type DistanceVector = BTreeMap<NodeID, Cost>;

struct Router {
    id: NodeID,
    routing_table: RoutingTable,
    dist_vec: DistanceVector,
}

impl Router {
    pub fn new(node_id: NodeID) -> Router {
        Router{
            id: node_id,
            routing_table: RoutingTable::new(),
            dist_vec: DistanceVector::new(),
        }
    }
    pub fn add_connection(&mut self, node_id: NodeID, neighbour_id: NodeID, cost: Cost) {
        self.routing_table.insert(node_id, RoutingTableEntry{
            neighbour_id: neighbour_id,
            cost: cost
        });
        self.dist_vec.insert(node_id, cost);
    }
    pub fn broadcast_dist_vec(&self) {
        for (node_id, cost) in &self.dist_vec {
            send(*node_id, Message::DistanceVector(*node_id, self.dist_vec.clone()));
        }
    }
    pub fn receive_msg(&mut self, link_id: NodeID, message: Message) {
        match message {
            Message::DistanceVector(dest_id, dist_vec) => {
                if dest_id != self.id {
                    send(dest_id, Message::DistanceVector(dest_id, dist_vec));
                } else {
                    self.receive_dist_vec(link_id, dist_vec);
                }
            },
            Message::Heartbeat(dest_id, from_id) => {
                if dest_id != self.id {
                    send(dest_id, message);
                } else {
                    self.receive_heartbeat(from_id);
                }
            },
            Message::SendMsg(dest_id, msg) => {
                if dest_id == self.id {
                    hprintln!("Got message!"); 
                } else {
                    send(dest_id, message);
                }
            }
        }    
    }
    pub fn receive_heartbeat(&self, from_id: NodeID) {
        hprintln!("Got heartbeat from node {:?}", from_id).unwrap();
    }
    pub fn receive_dist_vec(&mut self, link_id: NodeID, dist_vec: DistanceVector) {
        for (node_id, cost) in dist_vec {
            match self.dist_vec.get(&node_id) {
                // If seen route before, check if better
                Some(current_cost) => {
                    let new_cost = cost + 1;
                    if new_cost < *current_cost {
                        self.add_connection(node_id, link_id, new_cost);
                    }
                },
                // If new route, then save
                None => {
                    self.add_connection(node_id, link_id, cost);
                }
            }
        }
    }
}

fn send(node_id: NodeID, msg: Message) {
    hprintln!("Sending message!").unwrap();
}

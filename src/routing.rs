#![allow(dead_code)]
use std::cmp::Ordering;
use crate::constants::{N_BUCKETS, K_PARAM};
use crate::key::{Distance, Key};

use log::{info, warn};
use serde::{Serialize, Deserialize};

#[derive(Hash, Eq, PartialEq, Debug, Clone, Serialize, Deserialize)]
pub struct NodeInfo {
    pub id:Key,
    pub addr:String,
    pub net_id:String,
}

#[derive(Debug, Clone)]
pub struct RoutingTable {
    node_info:NodeInfo,
    buckets:Vec<Vec<NodeInfo>>
}

#[derive(Eq, Hash, Clone, Debug, Serialize, Deserialize)]
pub struct NodeAndDistance(pub NodeInfo, pub Distance);

impl PartialEq for NodeAndDistance {
    fn eq(&self, other: &NodeAndDistance) -> bool {
        self.1.eq(&other.1)
    }
}

impl PartialOrd for NodeAndDistance {
    fn partial_cmp(&self, other: &NodeAndDistance) -> Option<Ordering> {
        Some(other.1.cmp(&self.1))
    }
}

impl Ord for NodeAndDistance {
    fn cmp(&self, other:&NodeAndDistance) -> Ordering {
        other.1.cmp(&self.1)
    }
}

impl RoutingTable {
    pub fn new(node_info:NodeInfo) -> RoutingTable {
        let mut buckets = Vec::new();
        for _ in 0..N_BUCKETS {
            buckets.push(Vec::new());
        }
        let mut ret = RoutingTable {
            node_info: node_info.clone(),
            buckets: buckets
        };
        ret.update(node_info.clone());
        ret
    }

    /// Returns a clone of inner bucket instance for debugging and unit testing purposes
    pub fn _get_buckets(&self) -> Vec<Vec<NodeInfo>> {
        self.buckets.clone()
    }

    pub fn update(&mut self, node_info:NodeInfo){
        let bucket_index = self.lookup_bucket_index(node_info.id);
        let bucket = &mut self.buckets[bucket_index];
        let node_index = bucket.iter().position(|x| x.id == node_info.id);
        match node_index {
            Some(i) => {
                let temp = bucket.remove(i);
                bucket.push(temp);
            }
            None => {
                if bucket.len() < K_PARAM {
                    bucket.push(node_info);
                } else {
                
                }
            }
        }
    }

    pub fn closest_nodes(&self, item:Key, count:usize) -> Vec<NodeAndDistance> {
        if count == 0 {
            return Vec::new();
        }
        
        let mut ret = Vec::with_capacity(count);
        for bucket in &self.buckets {
            for node_info in bucket {
                ret.push(NodeAndDistance(node_info.clone(), node_info.id.dst(item)));
            }
        }

        ret.sort_by(|a,b| a.1.cmp(&b.1));
        ret.truncate(count);
        ret
    }

    pub fn remove(&mut self, node_info: &NodeInfo){
        let bucket_index = self.lookup_bucket_index(node_info.id);
        if let Some(item_index) = self.buckets[bucket_index].iter().position(|x| x == node_info){
            self.buckets[bucket_index].remove(item_index);
        } else {
            warn!("Tried to remove routing entry that doesn't exist");
        }
    }

    fn lookup_bucket_index(&self, item:Key) -> usize {
        self.node_info.id.dst(item).zeroes_in_prefix()
    }

    pub fn print(&self){
        info!("{:?}", self.buckets);
    }
}

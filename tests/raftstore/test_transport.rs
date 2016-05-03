// Copyright 2016 PingCAP, Inc.
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// See the License for the specific language governing permissions and
// limitations under the License.

use std::sync::Arc;
use super::cluster::{Cluster, Simulator};
use super::node::new_node_cluster;
use super::server::new_server_cluster;
use super::util::must_get_equal;
use std::collections::HashSet;

fn leader_in_majority_split(leader: u64, count: u64) -> (HashSet<u64>, HashSet<u64>) {
    if leader <= (count + 1) / 2 {
        ((1..count / 2 + 2).collect(), (count / 2 + 2..count + 1).collect())
    } else {
        (((count + 1) / 2..count + 1).collect(), (1..(count + 1) / 2).collect())
    }
}

fn leader_in_minority_split(leader: u64, count: u64) -> (HashSet<u64>, HashSet<u64>) {
    let mut vec: Vec<u64> = (1..count + 1).collect();
    vec.swap(0, (leader - 1) as usize);
    let ucount = count as usize;
    let s1 = vec[0..(ucount - 1) / 2].into_iter().cloned().collect();
    let s2 = vec[(ucount - 1) / 2..ucount].into_iter().cloned().collect();
    (s1, s2)
}

fn test_partition_write<T: Simulator>(cluster: &mut Cluster<T>, count: u64) {
    cluster.bootstrap_region().expect("");
    cluster.start();

    let (key, value) = (b"k1", b"v1");
    let region_id = cluster.get_region_id(key);
    let leader = cluster.leader_of_region(region_id).unwrap();

    // leader in majority, partition doesn't affect write/read
    let (s1, s2) = leader_in_majority_split(leader, count);
    cluster.partition(Arc::new(s1), Arc::new(s2));
    cluster.must_put(key, value);
    assert_eq!(cluster.get(key), Some(value.to_vec()));
    assert_eq!(cluster.leader_of_region(region_id).unwrap(), leader);
    cluster.reset_transport_hooks();

    // partition happended and leader in minority, new leader should be elected
    let (s1, s2) = leader_in_minority_split(leader, count);
    cluster.partition(Arc::new(s1), Arc::new(s2));
    cluster.must_put(key, b"v2");
    assert_eq!(cluster.get(key), Some(b"v2".to_vec()));
    assert!(cluster.leader_of_region(region_id).unwrap() != leader);
    cluster.must_put(b"k2", b"v2");

    // when network recover, old leader should sync data
    cluster.reset_transport_hooks();
    must_get_equal(&cluster.get_engine(leader), b"k2", b"v2");
    must_get_equal(&cluster.get_engine(leader), key, b"v2");
}

#[test]
fn test_node_partition_write() {
    let mut cluster = new_node_cluster(0, 5);
    test_partition_write(&mut cluster, 5);
}

#[test]
fn test_server_partition_write() {
    let mut cluster = new_server_cluster(0, 5);
    test_partition_write(&mut cluster, 5);
}

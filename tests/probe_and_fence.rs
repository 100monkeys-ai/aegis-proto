//! `Probe` and `Fence` (AEGIS ADR-152 D1, D6): the membership RPCs on
//! `NodeClusterService`. A probe answers which process of a node is alive
//! and in what state; a fence names the incarnation it stops.

use aegis_orchestrator_proto::aegis::cluster::v1::node_cluster_service_server::NodeClusterService;
use aegis_orchestrator_proto::aegis::cluster::v1::{
    FenceInner, FenceResponse, MemberState, ProbeInner, ProbeResponse,
};
use prost::Message;

/// The service declares both RPCs: this does not compile without them.
#[allow(dead_code)]
fn the_service_declares_probe_and_fence<T: NodeClusterService>() {
    let _ = T::probe;
    let _ = T::fence;
}

#[test]
fn a_probe_answer_round_trips_every_field() {
    let sent = ProbeResponse {
        node_id: "8c4e7f3a-1d2b-4c5e-9f6a-7b8c9d0e1f2a".to_string(),
        incarnation: 7,
        state: MemberState::Draining.into(),
        last_renewed_at: Some(prost_types::Timestamp {
            seconds: 1_795_000_000,
            nanos: 250_000_000,
        }),
    };
    let received = ProbeResponse::decode(sent.encode_to_vec().as_slice()).unwrap();
    assert_eq!(received, sent);
    assert_eq!(received.state(), MemberState::Draining);
}

/// A node that has never renewed answers with no renewal time, and a
/// reader that sees no state reads it as unspecified, never as active.
#[test]
fn a_probe_answer_before_the_first_renewal_carries_no_renewal_time() {
    let sent = ProbeResponse {
        node_id: "n".to_string(),
        incarnation: 1,
        state: 0,
        last_renewed_at: None,
    };
    let received = ProbeResponse::decode(sent.encode_to_vec().as_slice()).unwrap();
    assert!(received.last_renewed_at.is_none());
    assert_eq!(received.state(), MemberState::Unspecified);
}

#[test]
fn a_probe_names_the_prober_and_its_incarnation() {
    let sent = ProbeInner {
        node_id: "prober".to_string(),
        incarnation: 3,
    };
    assert_eq!(
        ProbeInner::decode(sent.encode_to_vec().as_slice()).unwrap(),
        sent
    );
}

#[test]
fn a_fence_names_its_target_incarnation_and_reason() {
    let sent = FenceInner {
        target_node_id: "target".to_string(),
        target_incarnation: 12,
        reason: "db_partition_grace_elapsed".to_string(),
    };
    assert_eq!(
        FenceInner::decode(sent.encode_to_vec().as_slice()).unwrap(),
        sent
    );
    let ack = FenceResponse {
        fenced: true,
        incarnation: 12,
    };
    assert_eq!(
        FenceResponse::decode(ack.encode_to_vec().as_slice()).unwrap(),
        ack
    );
}

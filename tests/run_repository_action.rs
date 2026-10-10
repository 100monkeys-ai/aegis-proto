//! `RunRepositoryAction` (AEGIS ADR-141 F5): the workflow interpreter's own
//! repository steps (`diff`, `commit`, `land`) on the repository a workflow
//! run holds, answered with the commit, the work branch, the binding's ref,
//! the diff and, on a refusal, the sentence.

use aegis_orchestrator_proto::aegis::runtime::v1::aegis_runtime_server::AegisRuntime;
use aegis_orchestrator_proto::aegis::runtime::v1::{
    RunRepositoryActionRequest, RunRepositoryActionResponse,
};
use prost::Message;

/// The request with only its required fields: what a caller sends for
/// `diff` and `land`, which carry no message.
#[derive(Clone, PartialEq, prost::Message)]
struct RequestWithoutOptionals {
    #[prost(string, tag = "1")]
    workflow_execution_id: String,
    #[prost(string, tag = "2")]
    action: String,
}

/// The response with only its required fields: a refusal-free answer that
/// names no commit and carries no diff.
#[derive(Clone, PartialEq, prost::Message)]
struct ResponseWithoutOptionals {
    #[prost(string, tag = "2")]
    branch: String,
    #[prost(string, tag = "3")]
    r#ref: String,
}

/// The service declares the RPC: this does not compile without it.
#[allow(dead_code)]
fn the_service_declares_run_repository_action<T: AegisRuntime>() {
    let _ = T::run_repository_action;
}

#[test]
fn a_request_round_trips_every_field() {
    let sent = RunRepositoryActionRequest {
        workflow_execution_id: "0b6f4c1e-8d2a-4f5b-9c3d-7e1a2b3c4d5e".to_string(),
        action: "commit".to_string(),
        message: Some("the-forge: name code-quality-judge".to_string()),
    };

    let received =
        RunRepositoryActionRequest::decode(sent.encode_to_vec().as_slice()).expect("decodes");

    assert_eq!(received.workflow_execution_id, sent.workflow_execution_id);
    assert_eq!(received.action, "commit");
    assert_eq!(
        received.message.as_deref(),
        Some("the-forge: name code-quality-judge")
    );
    assert_eq!(received, sent);
}

#[test]
fn a_response_round_trips_every_field() {
    let sent = RunRepositoryActionResponse {
        commit_sha: Some("eaa80dc2e728972fad79fe77e7b405913a31c501".to_string()),
        branch: "aegis/run-0b6f4c1e".to_string(),
        r#ref: "main".to_string(),
        diff: Some("diff --git a/x b/x\n".to_string()),
        sentence: Some("this run holds no repository".to_string()),
    };

    let received =
        RunRepositoryActionResponse::decode(sent.encode_to_vec().as_slice()).expect("decodes");

    assert_eq!(
        received.commit_sha.as_deref(),
        Some("eaa80dc2e728972fad79fe77e7b405913a31c501")
    );
    assert_eq!(received.branch, "aegis/run-0b6f4c1e");
    assert_eq!(received.r#ref, "main");
    assert_eq!(received.diff.as_deref(), Some("diff --git a/x b/x\n"));
    assert_eq!(
        received.sentence.as_deref(),
        Some("this run holds no repository")
    );
    assert_eq!(received, sent);
}

#[test]
fn a_request_encoded_without_the_message_decodes_with_it_absent() {
    let bytes = RequestWithoutOptionals {
        workflow_execution_id: "run-1".to_string(),
        action: "diff".to_string(),
    }
    .encode_to_vec();

    let received = RunRepositoryActionRequest::decode(bytes.as_slice()).expect("decodes");

    assert_eq!(received.workflow_execution_id, "run-1");
    assert_eq!(received.action, "diff");
    assert_eq!(
        received.message, None,
        "a request sent without a message decoded with one"
    );
}

#[test]
fn a_response_encoded_without_the_optionals_decodes_with_them_absent() {
    let bytes = ResponseWithoutOptionals {
        branch: "aegis/run-1".to_string(),
        r#ref: "main".to_string(),
    }
    .encode_to_vec();

    let received = RunRepositoryActionResponse::decode(bytes.as_slice()).expect("decodes");

    assert_eq!(received.branch, "aegis/run-1");
    assert_eq!(received.r#ref, "main");
    assert_eq!(received.commit_sha, None);
    assert_eq!(received.diff, None);
    assert_eq!(received.sentence, None);
}

#[test]
fn an_empty_optional_is_on_the_wire_and_distinct_from_absent() {
    let empty = RunRepositoryActionRequest {
        workflow_execution_id: "run-1".to_string(),
        action: "commit".to_string(),
        message: Some(String::new()),
    };
    let absent = RunRepositoryActionRequest {
        message: None,
        ..empty.clone()
    };

    assert_ne!(
        empty.encode_to_vec(),
        absent.encode_to_vec(),
        "an empty message and no message encode the same"
    );
    let received =
        RunRepositoryActionRequest::decode(empty.encode_to_vec().as_slice()).expect("decodes");
    assert_eq!(received.message.as_deref(), Some(""));
}

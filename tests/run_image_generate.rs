//! `RunImageGenerate` (AEGIS ADR-143 I5a): the workflow interpreter's
//! `image_generate` action, run by the orchestrator into the workflow
//! execution's volume, answered with the files it wrote or, when nothing was
//! generated, the sentence.

use aegis_orchestrator_proto::aegis::runtime::v1::aegis_runtime_server::AegisRuntime;
use aegis_orchestrator_proto::aegis::runtime::v1::{
    GeneratedFile, RunImageGenerateRequest, RunImageGenerateResponse,
};
use prost::Message;

/// The request with only its required fields: a prompt and nothing else, so
/// the orchestrator's defaults apply.
#[derive(Clone, PartialEq, prost::Message)]
struct RequestWithoutOptionals {
    #[prost(string, tag = "1")]
    workflow_execution_id: String,
    #[prost(string, tag = "2")]
    prompt: String,
}

/// The response with no field set: what an answer with no files and no
/// sentence looks like on the wire.
#[derive(Clone, PartialEq, prost::Message)]
struct EmptyResponse {}

/// The service declares the RPC: this does not compile without it.
#[allow(dead_code)]
fn the_service_declares_run_image_generate<T: AegisRuntime>() {
    let _ = T::run_image_generate;
}

fn a_file(n: u32) -> GeneratedFile {
    GeneratedFile {
        volume_id: "5c1d2e3f-4a5b-4c6d-8e7f-9a0b1c2d3e4f".to_string(),
        path: format!("generated/{n}.png"),
        content_type: "image/png".to_string(),
        size_bytes: 1_048_576 + u64::from(n),
    }
}

#[test]
fn a_request_round_trips_every_field() {
    let sent = RunImageGenerateRequest {
        workflow_execution_id: "0b6f4c1e-8d2a-4f5b-9c3d-7e1a2b3c4d5e".to_string(),
        prompt: "a small grey monkey reading a book under a tree".to_string(),
        alias: Some("image".to_string()),
        size: Some("1024x1024".to_string()),
        count: Some(4),
        path: Some("covers/".to_string()),
    };

    let received =
        RunImageGenerateRequest::decode(sent.encode_to_vec().as_slice()).expect("decodes");

    assert_eq!(received.workflow_execution_id, sent.workflow_execution_id);
    assert_eq!(
        received.prompt,
        "a small grey monkey reading a book under a tree"
    );
    assert_eq!(received.alias.as_deref(), Some("image"));
    assert_eq!(received.size.as_deref(), Some("1024x1024"));
    assert_eq!(received.count, Some(4u32));
    assert_eq!(received.path.as_deref(), Some("covers/"));
    assert_eq!(received, sent);
}

#[test]
fn a_request_encoded_with_only_the_prompt_decodes_with_the_optionals_absent() {
    let bytes = RequestWithoutOptionals {
        workflow_execution_id: "run-1".to_string(),
        prompt: "a lighthouse at dusk".to_string(),
    }
    .encode_to_vec();

    let received = RunImageGenerateRequest::decode(bytes.as_slice()).expect("decodes");

    assert_eq!(received.workflow_execution_id, "run-1");
    assert_eq!(received.prompt, "a lighthouse at dusk");
    assert_eq!(received.alias, None, "alias decoded present");
    assert_eq!(received.size, None, "size decoded present");
    assert_eq!(received.count, None, "count decoded present");
    assert_eq!(received.path, None, "path decoded present");
}

#[test]
fn a_zero_count_is_on_the_wire_and_distinct_from_absent() {
    let zero = RunImageGenerateRequest {
        workflow_execution_id: "run-1".to_string(),
        prompt: "p".to_string(),
        count: Some(0),
        ..Default::default()
    };
    let absent = RunImageGenerateRequest {
        count: None,
        ..zero.clone()
    };

    assert_ne!(
        zero.encode_to_vec(),
        absent.encode_to_vec(),
        "a count of 0 and no count encode the same"
    );
    let received =
        RunImageGenerateRequest::decode(zero.encode_to_vec().as_slice()).expect("decodes");
    assert_eq!(received.count, Some(0));
}

#[test]
fn a_response_round_trips_its_files_in_order() {
    let sent = RunImageGenerateResponse {
        files: vec![a_file(1), a_file(2)],
        sentence: None,
    };

    let received =
        RunImageGenerateResponse::decode(sent.encode_to_vec().as_slice()).expect("decodes");

    assert_eq!(received.files.len(), 2);
    let first = &received.files[0];
    assert_eq!(first.volume_id, "5c1d2e3f-4a5b-4c6d-8e7f-9a0b1c2d3e4f");
    assert_eq!(first.path, "generated/1.png");
    assert_eq!(first.content_type, "image/png");
    assert_eq!(first.size_bytes, 1_048_577u64);
    assert_eq!(received.files[1].path, "generated/2.png");
    assert_eq!(received.sentence, None);
    assert_eq!(received, sent);
}

#[test]
fn a_refusal_carries_the_sentence_and_no_files() {
    let sent = RunImageGenerateResponse {
        files: Vec::new(),
        sentence: Some("this node has no image generation model".to_string()),
    };

    let received =
        RunImageGenerateResponse::decode(sent.encode_to_vec().as_slice()).expect("decodes");

    assert!(received.files.is_empty());
    assert_eq!(
        received.sentence.as_deref(),
        Some("this node has no image generation model")
    );
}

#[test]
fn an_empty_response_decodes_with_no_files_and_no_sentence() {
    let received = RunImageGenerateResponse::decode(EmptyResponse {}.encode_to_vec().as_slice())
        .expect("decodes");

    assert!(received.files.is_empty());
    assert_eq!(received.sentence, None);
}

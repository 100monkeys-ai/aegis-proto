//! `InvokeToolResponse.grounding_json` (AEGIS ADR-132 H9a): the `_grounding`
//! object a remote server answered on `initialize` travels beside the
//! `tools/call` result, never inside `result_json`.

use aegis_orchestrator_proto::aegis::seal_gateway::v1::InvokeToolResponse;
use prost::Message;

/// `InvokeToolResponse` as it stood at `961bca97`, before field 2 existed:
/// the wire form an older gateway still sends.
#[derive(Clone, PartialEq, prost::Message)]
struct InvokeToolResponseBeforeGrounding {
    #[prost(string, tag = "1")]
    result_json: String,
}

#[test]
fn grounding_json_round_trips_beside_result_json() {
    let sent = InvokeToolResponse {
        result_json: r#"{"content":[{"type":"text","text":"page"}],"isError":false}"#.to_string(),
        grounding_json: r#"{"instance":"main","workspace":"notes"}"#.to_string(),
    };

    let bytes = sent.encode_to_vec();
    let received = InvokeToolResponse::decode(bytes.as_slice()).expect("decodes");

    assert_eq!(
        received.grounding_json, sent.grounding_json,
        "grounding_json did not survive encode and decode"
    );
    assert_eq!(
        received.result_json, sent.result_json,
        "result_json did not survive encode and decode beside grounding_json"
    );
    assert!(
        !received.result_json.contains("instance"),
        "the grounding leaked into result_json"
    );
}

#[test]
fn a_response_encoded_before_the_field_existed_decodes_with_grounding_json_empty() {
    let old = InvokeToolResponseBeforeGrounding {
        result_json: r#"{"isError":false}"#.to_string(),
    };
    let bytes = old.encode_to_vec();

    // Field 1, wire type 2 (tag byte 0x0a), length 17, then the UTF-8 text:
    // the old encoding carries nothing for field 2.
    let mut expected = vec![0x0a, 17];
    expected.extend_from_slice(br#"{"isError":false}"#);
    assert_eq!(
        bytes, expected,
        "the old wire form is not what 961bca97 sent"
    );

    let received = InvokeToolResponse::decode(bytes.as_slice()).expect("decodes");

    assert_eq!(received.result_json, r#"{"isError":false}"#);
    assert_eq!(
        received.grounding_json, "",
        "an old response decoded with a grounding it never carried"
    );
}

#[test]
fn an_empty_grounding_json_is_not_on_the_wire() {
    let sent = InvokeToolResponse {
        result_json: r#"{"isError":false}"#.to_string(),
        grounding_json: String::new(),
    };
    let old = InvokeToolResponseBeforeGrounding {
        result_json: r#"{"isError":false}"#.to_string(),
    };

    assert_eq!(
        sent.encode_to_vec(),
        old.encode_to_vec(),
        "a response with no grounding is not byte-identical to the old wire form"
    );
}

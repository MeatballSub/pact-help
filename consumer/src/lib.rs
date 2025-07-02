mod pb {
    include!("../../proto/generated/rust/list_appender.rs");
}

#[cfg(test)]
mod tests {
    use std::path::Path;

    use crate::pb::AppendRequest;
    use crate::pb::AppendResponse;
    use pact_consumer::mock_server::StartMockServerAsync;
    use pact_consumer::prelude::*;
    use serde_json::json;

    use crate::pb::list_appender_client::ListAppenderClient;

    #[tokio::test(flavor = "multi_thread", worker_threads = 1)]
    async fn test_proto_client() {
        let mut pact_builder =
            PactBuilderAsync::new_v4("multi-grpc-consumer-rust", "list-appender-provider");

        let mock_server = pact_builder
            // Tell Pact we need the Protobuf plugin
            .using_plugin("protobuf", None)
            .await
            .synchronous_message_interaction("something to append", |mut i| async move {
                i.contents_from(json!({
                      "pact:proto": Path::new("../proto/list_appender.proto").canonicalize().unwrap().to_string_lossy().to_string(),
                      "pact:content-type": "application/protobuf",
                      "pact:proto-service": "ListAppender/append",

                      "request": {
                        "start": 0,
                        "additional":[1,2,3]
                      },

                      "response": {
                        "value": [0,1,2,3]
                      }
                    }))
                .await;
                i
            })
            .await
            .synchronous_message_interaction("nothing to append", |mut i| async move {
                i.contents_from(json!({
                      "pact:proto": Path::new("../proto/list_appender.proto").canonicalize().unwrap().to_string_lossy().to_string(),
                      "pact:content-type": "application/protobuf",
                      "pact:proto-service": "ListAppender/append",

                      "request": {
                        "start": 0,
                        "additional":[]
                      },

                      "response": {
                        "value": [0]
                      }
                    }))
                .await;
                i
            })
            .await
            // Start a mock server using gRPC transport
            .start_mock_server_async(Some("protobuf/transport/grpc"), None)
            .await;

        let url = mock_server.url();
        let mut client = ListAppenderClient::connect(url.to_string()).await.unwrap();

        for message in pact_builder.synchronous_messages() {
            let request: AppendRequest =
                prost::Message::decode(message.request.contents.value().unwrap()).unwrap();
            println!(
                "Request:\n{}",
                serde_json::to_string_pretty(&request).unwrap()
            );

            let mocked_actual_response = client
                .append(request)
                .await
                .expect("Some error unwrapping Result<AppendResponse>")
                .into_inner();
            println!(
                "Mocked Actual Response:\n{}",
                serde_json::to_string_pretty(&mocked_actual_response).unwrap()
            );

            let pact_expected_response: AppendResponse =
                prost::Message::decode(message.response.first().unwrap().contents.value().unwrap())
                    .unwrap();
            println!(
                "Pact Expected Response:\n{}",
                serde_json::to_string_pretty(&pact_expected_response).unwrap()
            );

            assert_eq!(mocked_actual_response, pact_expected_response);
            println!("\n\n*****************\nAssertion Success\n*****************\n\n");
        }
    }
}

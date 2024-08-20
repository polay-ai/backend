use crate::ollyllm::ollyllm_service_client::OllyllmServiceClient;
use crate::ollyllm::{ReportSpanRequest, Span, TestExecutionRequest, VersionedTest};
use prost_types::Timestamp;
use tonic::transport::Channel;

pub struct Client {
    client: OllyllmServiceClient<Channel>,
}

impl Client {
    pub async fn new() -> Result<Self, tonic::transport::Error> {
        let client = OllyllmServiceClient::connect("http://[::1]:50051").await?;
        Ok(Client { client })
    }

    pub async fn send_dummy_span_creation_request(
        &mut self,
    ) -> Result<tonic::Response<()>, tonic::Status> {
        let span = Span {
            id: "12345abcd".to_string(),
            start_timestamp: Some(Timestamp {
                seconds: 10,
                nanos: 1,
            }),
            end_timestamp: None,
            operation_name: "start call to openai".to_string(),
            parent_id: "parent_of_12345abcd".to_string(),
            trace_id: "trace_uuid".to_string(),
            external_uuid: String::new(),
        };

        let span_request: tonic::Request<ReportSpanRequest> =
            tonic::Request::new(ReportSpanRequest { spans: vec![span] });
        self.client.report_span(span_request).await
    }

    pub async fn send_dummy_test_execution_request(
        &mut self,
    ) -> Result<tonic::Response<()>, tonic::Status> {
        let test_request: tonic::Request<TestExecutionRequest> =
            tonic::Request::new(TestExecutionRequest {
                session_id: 1,
                versioned_test: Some(VersionedTest { id: 1, version: 1 }),
                request_timestamp: Some(Timestamp {
                    seconds: 100,
                    nanos: 10,
                }),
                test_input: Vec::new(),
            });
        self.client.queue_test(test_request).await
    }
}

#[cfg(test)]
mod tests {
    use tokio::{sync::oneshot, task::JoinHandle};

    use super::*;
    use crate::server::RpcServer;

    #[tokio::test]
    async fn test_queue_span_creation_request() {
        let (tx, rx): (oneshot::Sender<()>, oneshot::Receiver<()>) = oneshot::channel();

        let server_handle: JoinHandle<()> = tokio::spawn(async move {
            let addr: core::net::SocketAddr = "[::1]:50051".parse().unwrap();
            let server: RpcServer = RpcServer::new(addr).await;

            tx.send(()).unwrap();
            server.serve().await.unwrap();
        });

        rx.await.unwrap();

        let mut client: Client = Client::new().await.unwrap();
        let response: Result<tonic::Response<()>, tonic::Status> =
            client.send_dummy_span_creation_request().await;

        assert!(response.is_ok());

        server_handle.abort();
    }

    #[tokio::test]
    async fn test_queue_test_execution_request() {
        let (tx, rx): (oneshot::Sender<()>, oneshot::Receiver<()>) = oneshot::channel();

        let server_handle: JoinHandle<()> = tokio::spawn(async move {
            let addr: core::net::SocketAddr = "[::1]:50051".parse().unwrap();
            let server: RpcServer = RpcServer::new(addr).await;

            tx.send(()).unwrap();
            server.serve().await.unwrap();
        });

        rx.await.unwrap();

        let mut client: Client = Client::new().await.unwrap();
        let response: Result<tonic::Response<()>, tonic::Status> =
            client.send_dummy_test_execution_request().await;

        assert!(response.is_ok());

        server_handle.abort();
    }
}

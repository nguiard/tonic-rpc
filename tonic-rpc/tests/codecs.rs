use tonic_rpc::tonic_rpc;

#[tonic_rpc(json)]
trait MathJson {
    fn add(args: (i32, i32)) -> i32;
}

#[tonic_rpc(cbor)]
trait MathCbor {
    fn add(args: (i32, i32)) -> i32;
}

#[tonic_rpc(bincode)]
trait MathBincode {
    fn add(args: (i32, i32)) -> i32;
}

#[tonic_rpc(messagepack)]
trait MathMessagePack {
    fn add(args: (i32, i32)) -> i32;
}

type State = ();

#[tonic::async_trait]
impl math_json_server::MathJson for State {
    async fn add(
        &self,
        request: tonic::Request<(i32, i32)>,
    ) -> Result<tonic::Response<i32>, tonic::Status> {
        let (x, y) = request.into_inner();
        Ok(tonic::Response::new(x + y))
    }
}

#[tonic::async_trait]
impl math_cbor_server::MathCbor for State {
    async fn add(
        &self,
        request: tonic::Request<(i32, i32)>,
    ) -> Result<tonic::Response<i32>, tonic::Status> {
        let (x, y) = request.into_inner();
        Ok(tonic::Response::new(x + y))
    }
}

#[tonic::async_trait]
impl math_bincode_server::MathBincode for State {
    async fn add(
        &self,
        request: tonic::Request<(i32, i32)>,
    ) -> Result<tonic::Response<i32>, tonic::Status> {
        let (x, y) = request.into_inner();
        Ok(tonic::Response::new(x + y))
    }
}

#[tonic::async_trait]
impl math_message_pack_server::MathMessagePack for State {
    async fn add(
        &self,
        request: tonic::Request<(i32, i32)>,
    ) -> Result<tonic::Response<i32>, tonic::Status> {
        let (x, y) = request.into_inner();
        Ok(tonic::Response::new(x + y))
    }
}

pub async fn run_server() -> u16 {
    let mut listener = tokio::net::TcpListener::bind("[::1]:0").await.unwrap();
    let port = listener.local_addr().unwrap().port();

    tokio::spawn(async move {
        tonic::transport::Server::builder()
            .add_service(math_json_server::MathJsonServer::new(()))
            .add_service(math_cbor_server::MathCborServer::new(()))
            .add_service(math_bincode_server::MathBincodeServer::new(()))
            .add_service(math_message_pack_server::MathMessagePackServer::new(()))
            .serve_with_incoming(listener.incoming())
            .await
            .unwrap();
    });
    port
}

#[tokio::test]
async fn test_json_codec() {
    let port = run_server().await;
    // Wait for server to start
    tokio::time::delay_for(std::time::Duration::from_millis(1)).await;
    let mut client = math_json_client::MathJsonClient::connect(format!("http://[::1]:{}", port))
        .await
        .expect("Failed to connect");

    assert_eq!(
        77,
        client
            .add(tonic::Request::new((42_i32, 35_i32)))
            .await
            .expect("Failed to send request")
            .into_inner()
    );
}

#[tokio::test]
async fn test_cbor_codec() {
    let port = run_server().await;
    // Wait for server to start
    tokio::time::delay_for(std::time::Duration::from_millis(1)).await;
    let mut client = math_cbor_client::MathCborClient::connect(format!("http://[::1]:{}", port))
        .await
        .expect("Failed to connect");

    assert_eq!(
        77,
        client
            .add(tonic::Request::new((42_i32, 35_i32)))
            .await
            .expect("Failed to send request")
            .into_inner()
    );
}

#[tokio::test]
async fn test_bincode_codec() {
    let port = run_server().await;
    // Wait for server to start
    tokio::time::delay_for(std::time::Duration::from_millis(1)).await;
    let mut client =
        math_bincode_client::MathBincodeClient::connect(format!("http://[::1]:{}", port))
            .await
            .expect("Failed to connect");

    assert_eq!(
        77,
        client
            .add(tonic::Request::new((42_i32, 35_i32)))
            .await
            .expect("Failed to send request")
            .into_inner()
    );
}

#[tokio::test]
async fn test_message_pack_codec() {
    let port = run_server().await;
    // Wait for server to start
    tokio::time::delay_for(std::time::Duration::from_millis(1)).await;
    let mut client =
        math_message_pack_client::MathMessagePackClient::connect(format!("http://[::1]:{}", port))
            .await
            .expect("Failed to connect");

    assert_eq!(
        77,
        client
            .add(tonic::Request::new((42_i32, 35_i32)))
            .await
            .expect("Failed to send request")
            .into_inner()
    );
}
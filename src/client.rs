use payments::bitcoin_client::BitcoinClient;
use payments::BtcPaymentRequest;

pub mod payments {
    tonic::include_proto!("payments");
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut client:BitcoinClient<tonic::transport::Channel> = BitcoinClient::connect("http://[::1]:50051").await?;

    let request:tonic::Request<BtcPaymentRequest> = tonic::Request::new(
        BtcPaymentRequest {
            from_addr: "123456".to_owned(),
            to_addr: "654321".to_owned(),
            amount: 22
        }
    );

    let response:tonic::Response<payments::BtcPaymentResponse> = client.send_payment(request).await?;

    println!("RESPONSE={:?}", response);

    Ok(())
}
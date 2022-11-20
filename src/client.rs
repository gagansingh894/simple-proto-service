use payments::bitcoin_client::BitcoinClient;
use payments::BtcPaymentRequest;

pub mod payments {
    tonic::include_proto!("payments");
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut client = BitcoinClient::connect(
        "http://[::1]:50051"
    ).await?;

    let request = tonic::Request::new(
        BtcPaymentRequest{
            from_addr: String::from("123456"),
            to_addr: String::from("654321"),
            amount: 10,
        }
    );

    let response = client.send_payment(request).await?;

    print!("response={:?}", response);

    Ok(())
}
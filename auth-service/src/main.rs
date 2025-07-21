use auth_service::Application;

#[tokio::main]
async fn main() {
    let app = Application::build("0.0.0.0:3000")
        .await
        .expect("Failed to build app");

    // copied the address from the app so we could
    // use it in the print statement below
    let local_addr = app.address.clone();
    app.run().await.expect("Failed to run app");

    println!("listening on {local_addr}");
}

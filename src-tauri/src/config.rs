pub fn get_client_credentials() -> (String, String) {
    let client_id = std::env::var("DCADMIN_CLIENT_ID").expect("DCADMIN_CLIENT_ID must be set");
    let client_secret =
        std::env::var("DCADMIN_CLIENT_SECRET").expect("DCADMIN_CLIENT_SECRET must be set");

    (client_id, client_secret)
}

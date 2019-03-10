use crate::clients::google::GoogleClient;

pub fn playground(google_client: &mut GoogleClient) {
    let token = google_client.get_access_token(vec![
        "https://www.googleapis.com/auth/calendar".to_string()
    ]);

    dbg!(token);
}

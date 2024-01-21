use std::collections::HashSet;
use notify_rust::Notification;
use reqwest::blocking::get;

fn send_get_request(channel: &str) -> Result<String, reqwest::Error> {
    let channel_url = format!("https:///twitch.tv/{}", channel);
    let response = get(channel_url)?.text()?;
    Ok(response)
}

fn notify(channel: &str) {
    let body = format!("{} is live.", channel);
    Notification::new()
        .summary("Totify")
        .body(body.as_str())
        .show()
        .unwrap();
}

fn main() {
    let channels = vec!["avril", "LCS", "lol_nemesis"];
    let mut live_channels = HashSet::new();

    loop {
        for channel in &channels {
            let http_resp = send_get_request(channel);
            if let Ok(resp) = http_resp {
                if resp.contains("isLiveBroadcast") {
                    if !live_channels.contains(channel) {
                        live_channels.insert(*channel);
                        notify(channel);
                    }
                } else {
                    live_channels.remove(channel);
                }
            } else {
                println!("Failed to get status for {}.", channel);
            }
        }
        std::thread::sleep(std::time::Duration::from_secs(60));
    }
}
use get_if_addrs::{get_if_addrs, IfAddr};
use serde::{Deserialize, Serialize};
use structopt::StructOpt;

const CLOUDFLARE_API_ENDPOINT: &str = "https://api.cloudflare.com/client/v4";

#[derive(Deserialize, Debug, structopt::StructOpt)]
#[structopt(
    name = "dyndns",
    about = "Update Cloudflare DNS records based on local IP address"
)]
struct CliOptions {
    #[structopt(short = "i", long = "interface", env = "INTERFACE")]
    pub interface: String,
    #[structopt(short = "z", long = "zone-id", env = "ZONE_ID")]
    pub zone_id: String,
    #[structopt(short = "a", long = "api-token", env = "API_TOKEN")]
    pub api_token: String,
}

fn get_ipv4_from_interface(interface_name: &str) -> Option<String> {
    let if_addrs = get_if_addrs().ok()?;

    if_addrs.into_iter().find_map(|iface| {
        if iface.name == interface_name {
            match iface.addr {
                IfAddr::V4(ipv4) => Some(ipv4.ip.to_string()),
                _ => None,
            }
        } else {
            None
        }
    })
}

#[derive(Deserialize, Debug)]
struct DnsRecord {
    id: String,
    name: String,
    r#type: String,
    content: String,
}

#[derive(Serialize)]
struct UpdateDnsRecord {
    content: String,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli_options = CliOptions::from_args();

    let interface_name = &cli_options.interface;

    // Get IPv4 address of the specified interface
    let ipv4 = match get_ipv4_from_interface(interface_name) {
        Some(ip) => ip,
        None => {
            eprintln!(
                "Could not find IPv4 address for interface: {}",
                interface_name
            );
            return Ok(());
        }
    };

    println!("Local IPv4: {}", ipv4);

    // Constants for Cloudflare API

    let zone_id: &str = &cli_options.zone_id;
    let cloudflare_api_token: String = format!("Bearer {}", cli_options.api_token);

    // Get current DNS A records from Cloudflare
    let url = format!(
        "{}/zones/{}/dns_records?type=A",
        CLOUDFLARE_API_ENDPOINT, zone_id
    );

    let body: String = ureq::get(&url)
        .set("Authorization", &cloudflare_api_token)
        .call()?
        .into_string()?;

    let response: Result<serde_json::Value, _> = serde_json::from_str(&body);

    let dns_records = match response {
        Ok(value) => value["result"]
            .as_array()
            .map(|records| {
                records
                    .iter()
                    .filter_map(|record| serde_json::from_value::<DnsRecord>(record.clone()).ok())
                    .collect::<Vec<DnsRecord>>()
            })
            .unwrap_or_default(),
        Err(err) => {
            eprintln!("Failed to fetch DNS records: {}", err);
            return Err(err.into());
        }
    };

    println!("Fetched DNS A records: {:?}", dns_records);

    // Check if the IPv4 address matches any of the A records
    let matches = dns_records.iter().any(|record| record.content == ipv4);

    if matches {
        println!("IPv4 address matches the current DNS A records. No update needed.");
        return Ok(());
    }

    println!("IPv4 address does not match any DNS A records. Updating...");

    // Update all DNS A records to the correct IPv4 address
    for record in dns_records {
        let update_record_url = format!(
            "{}/zones/{}/dns_records/{}",
            CLOUDFLARE_API_ENDPOINT, zone_id, record.id
        );

        let update_record = UpdateDnsRecord {
            content: ipv4.clone(),
        };

        println!(
            "Updating DNS record: {} with the IP {} for current: {}",
            record.id,
            record.content,
            ipv4.clone()
        );

        let update_response = ureq::patch(&update_record_url)
            .set("Authorization", &cloudflare_api_token)
            .set("Content-Type", "application/json")
            .send_json(&update_record);

        match update_response {
            Ok(resp) if resp.status() == 200 => {
                println!("Successfully updated DNS record: {:?} with new IP", record);
            }
            Ok(resp) => {
                eprintln!(
                    "Failed to update DNS record: {:?} - Status: {}",
                    record,
                    resp.status()
                );
            }
            Err(err) => {
                eprintln!("Failed to update DNS record: {:?} - Error: {}", record, err);
            }
        }
    }
    Ok(())
}

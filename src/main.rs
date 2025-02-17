use std::env;

use cloudflare::structs::Record;

mod ipify;
mod cloudflare;
mod records;

#[tokio::main]
async fn main() {
    let cloudflare_email = match env::var("CF_EMAIL") {
        Ok(v) => v,
        _ => panic!("La variable d'environnement CF_EMAIL n'est pas fixé")
    };

    let cloudflare_api_key = match env::var("CF_API_KEY") {
        Ok(v) => v,
        _ => panic!("La variable d'environnement CF_API_KEY n'est pas fixé")
    };

    let ipv4 = match ipify::get_public_ip().await {
        Ok(v) => v,
        _ => panic!("L'addresse ip publique n'a pu être trouvé.")
    };
    
    let records = records::get_records();

    for record in records {
        let rd = Record {
            content: Some(ipv4.clone()),
            comment: None,
            name: None,
            proxied: None,
            settings: None,
            ttl: None
        };

        let response = cloudflare::update::update_dns_record(
            rd, 
            &record[0],
            &record[1],
            &cloudflare_email,
            &cloudflare_api_key
        ).await;

        match response {
            true => println!("L'enregistrement {} a été mis à jours.", record[1]),
            false => println!("L'enregistrement {} n'a pas été mis à jours.", record[1])
        };
    }
}
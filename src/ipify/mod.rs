use reqwest::Error;

pub async fn get_public_ip() -> Result<String, Error> {
    // Faites une requête GET à un service qui renvoie l'adresse IP publique
    let response = reqwest::get("https://api.ipify.org")
        .await?
        .text()
        .await?;

    Ok(response)
}
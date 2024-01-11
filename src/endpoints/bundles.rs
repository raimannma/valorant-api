use crate::models::api_result::APIResult;
use crate::models::bundle::Bundle;
use crate::models::language::Language;
use url::Url;
use uuid::Uuid;

/// This function is used to get a list of bundles from the Valorant API.
///
/// # Arguments
///
/// * `client` - A reference to a reqwest::Client, which is used to send HTTP requests.
/// * `language` - An Option that may contain a Language. If Some, this language is used as a query parameter.
///
/// # Returns
///
/// This function returns a Result that, if Ok, contains a Vec of Bundles.
///
/// # Errors
///
/// This function will return an Err if the HTTP request fails for any reason, or if the response cannot be parsed into an `APIResult<Vec<Bundle>>`.
pub async fn get_bundles(
    client: &reqwest::Client,
    language: Option<Language>,
) -> reqwest::Result<Vec<Bundle>> {
    let mut url = Url::parse("https://valorant-api.com/v1/bundles").unwrap();
    if let Some(language) = language {
        url.query_pairs_mut()
            .append_pair("language", &format!("{}", language));
    }
    client
        .get(url)
        .send()
        .await?
        .json::<APIResult<Vec<_>>>()
        .await
        .map(|x| x.data)
}

/// This function is used to get a specific bundle from the Valorant API.
///
/// # Arguments
///
/// * `client` - A reference to a reqwest::Client, which is used to send HTTP requests.
/// * `bundle` - A Uuid that represents the unique identifier of the bundle.
/// * `language` - An Option that may contain a Language. If Some, this language is used as a query parameter.
///
/// # Returns
///
/// This function returns a Result that, if Ok, contains a Bundle.
///
/// # Errors
///
/// This function will return an Err if the HTTP request fails for any reason, or if the response cannot be parsed into an `APIResult<Bundle>`.
pub async fn get_bundle(
    client: &reqwest::Client,
    bundle: Uuid,
    language: Option<Language>,
) -> reqwest::Result<Bundle> {
    let mut url = Url::parse(&format!("https://valorant-api.com/v1/bundles/{}", bundle)).unwrap();
    if let Some(language) = language {
        url.query_pairs_mut()
            .append_pair("language", &format!("{}", language));
    }
    client
        .get(url)
        .send()
        .await?
        .json::<APIResult<_>>()
        .await
        .map(|x| x.data)
}

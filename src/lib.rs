use error::AppResult;
use reqwest::Method;
use serde::Deserialize;
use worker::{console_log, event, Env, ScheduleContext, ScheduledEvent};

pub mod error;

#[derive(Debug, Deserialize)]
struct SiteMap {
    #[serde(default, rename = "url")]
    urls: Vec<Url>,
}

#[derive(Debug, Deserialize)]
struct Url {
    loc: String,
}

const SITE_MAP_URL_VAR: &str = "SITE_MAP_URL";

#[event(scheduled)]
async fn run_scheduled(_event: ScheduledEvent, env: Env, _ctx: ScheduleContext) {
    console_error_panic_hook::set_once();

    let site_map_url = env.var(SITE_MAP_URL_VAR).unwrap().to_string();

    if let Err(err) = refresh_site(&site_map_url).await {
        console_log!("Error: {}", err);
    }
}

// #[event(fetch)]
// async fn fetch(_req: Request, _env: Env, _ctx: Context) -> worker::Result<Response> {
//     console_error_panic_hook::set_once();

//     let site_map_url = env.var(SITE_MAP_URL_VAR)?.to_string();
//     refresh_site(&site_map_url).await?;

//     Response::ok("OK")
// }

async fn refresh_site(site_map_url: &str) -> AppResult<()> {
    console_log!("Site map URL: {}", site_map_url);
    let site_map = get_site_map(site_map_url).await?;
    for url in site_map.urls {
        refresh_page(&url.loc).await?;
    }
    Ok(())
}

async fn refresh_page(url: &str) -> AppResult<()> {
    console_log!("Refreshing {}", url);
    let client = reqwest::Client::new();
    client
        .request(Method::GET, url)
        // .header(header::CACHE_CONTROL, "no-cache")
        .send()
        .await?;
    Ok(())
}

async fn get_site_map(site_map_url: &str) -> AppResult<SiteMap> {
    let res = reqwest::get(site_map_url).await?.text().await?;
    let site_map = quick_xml::de::from_str::<SiteMap>(&res)?;
    Ok(site_map)
}

use crate::fake_hash_map::FakeHashMap;
use serde::{de::DeserializeOwned, Deserialize};
use std::collections::HashMap;
use std::net::IpAddr;
pub mod errors;
mod fake_hash_map;

/// Summary Raw Struct
#[derive(Deserialize, Debug)]
pub struct SummaryRaw {
    /// Number of domains being blocked
    pub domains_being_blocked: u64,

    /// Number of DNS queries today
    pub dns_queries_today: u64,

    /// Number of Ads blocked today
    pub ads_blocked_today: u64,

    /// Percentage of queries blocked today
    pub ads_percentage_today: f64,

    /// Number of unique domains
    pub unique_domains: u64,

    /// Number of queries forwarded
    pub queries_forwarded: u64,

    /// Number of queries cached
    pub queries_cached: u64,

    /// Number of clients ever seen
    pub clients_ever_seen: u64,

    /// Number of unique clients
    pub unique_clients: u64,

    /// Number of DNS queries of all types
    pub dns_queries_all_types: u64,

    /// Number of NODATA replies
    #[serde(rename = "reply_NODATA")]
    pub reply_nodata: u64,

    /// Number of NXDOMAIN replies
    #[serde(rename = "reply_NXDOMAIN")]
    pub reply_nxdomain: u64,

    /// Number of CNAME replies
    #[serde(rename = "reply_CNAME")]
    pub reply_cname: u64,

    /// Number of IP replies
    #[serde(rename = "reply_IP")]
    pub reply_ip: u64,

    /// Privacy level
    pub privacy_level: u64,

    /// Pi Hole status
    pub status: String,
}

/// Summary Struct
#[derive(Deserialize, Debug)]
pub struct Summary {
    /// Formatted number of domains being blocked
    pub domains_being_blocked: String,

    /// Formatted number of DNS queries today
    pub dns_queries_today: String,

    /// Formatted number of Ads blocked today
    pub ads_blocked_today: String,

    /// Formatted percentage of queries blocked today
    pub ads_percentage_today: String,

    /// Formatted number of unique domains
    pub unique_domains: String,

    /// Formatted number of queries forwarded
    pub queries_forwarded: String,

    /// Formatted number of queries cached
    pub queries_cached: String,

    /// Formatted number of clients ever seen
    pub clients_ever_seen: String,

    /// Formatted number of unique clients
    pub unique_clients: String,

    /// Formatted number of DNS queries of all types
    pub dns_queries_all_types: String,

    /// Formatted number of NODATA replies
    #[serde(rename = "reply_NODATA")]
    pub reply_nodata: String,

    /// Formatted number of NXDOMAIN replies
    #[serde(rename = "reply_NXDOMAIN")]
    pub reply_nxdomain: String,

    /// Formatted number of CNAME replies
    #[serde(rename = "reply_CNAME")]
    pub reply_cname: String,

    /// Formatted number of IP replies
    #[serde(rename = "reply_IP")]
    pub reply_ip: String,

    /// Privacy level
    pub privacy_level: String,

    /// Pi Hole status
    pub status: String,
}

/// Over Time Data Struct
#[derive(Deserialize, Debug)]
pub struct OverTimeData {
    /// Mapping from time to number of domains
    #[serde(deserialize_with = "fake_hash_map::deserialize_fake_hash_map")]
    pub domains_over_time: HashMap<String, u64>,

    /// Mapping from time to number of ads
    #[serde(deserialize_with = "fake_hash_map::deserialize_fake_hash_map")]
    pub ads_over_time: HashMap<String, u64>,
}

/// Top Items Struct
#[derive(Deserialize, Debug)]
pub struct TopItems {
    /// Top queries mapping from domain to number of requests
    #[serde(deserialize_with = "fake_hash_map::deserialize_fake_hash_map")]
    pub top_queries: HashMap<String, u64>,

    /// Top ads mapping from domain to number of requests
    #[serde(deserialize_with = "fake_hash_map::deserialize_fake_hash_map")]
    pub top_ads: HashMap<String, u64>,
}

/// Top Clients Struct
#[derive(Deserialize, Debug)]
pub struct TopClients {
    /// Top sources mapping from "IP" or "hostname|IP" to number of requests.
    #[serde(deserialize_with = "fake_hash_map::deserialize_fake_hash_map")]
    pub top_sources: HashMap<String, u64>,
}

/// Top Clients Blocked Struct
#[derive(Deserialize, Debug)]
pub struct TopClientsBlocked {
    /// Top sources blocked mapping from "IP" or "hostname|IP" to number of blocked requests.
    #[serde(deserialize_with = "fake_hash_map::deserialize_fake_hash_map")]
    pub top_sources_blocked: HashMap<String, u64>,
}

/// Forward Destinations Struct
#[derive(Deserialize, Debug)]
pub struct ForwardDestinations {
    /// Forward destinations mapping from "human_readable_name|IP" to the percentage of requests answered.
    #[serde(deserialize_with = "fake_hash_map::deserialize_fake_hash_map")]
    pub forward_destinations: HashMap<String, f64>,
}

/// Query Types Struct
#[derive(Deserialize, Debug)]
pub struct QueryTypes {
    /// Query types mapping from query type (A, AAAA, PTR, etc.) to the percentage of queries which are of that type.
    #[serde(deserialize_with = "fake_hash_map::deserialize_fake_hash_map")]
    pub querytypes: HashMap<String, f64>,
}

/// Query Struct
#[derive(Deserialize, Debug)]
pub struct Query {
    /// Timestamp of query
    pub timestring: String,

    /// Type of query (A, AAAA, PTR, etc.)
    pub query_type: String,

    /// Requested domain name
    pub domain: String,

    /// Requesting client IP or hostname
    pub client: String,

    /// Status as String
    pub answer_type: String,
}

/// All Queries Struct
#[derive(Deserialize, Debug)]
pub struct AllQueries {
    /// List of queries
    pub data: Vec<Query>,
}

/// Status Struct
#[derive(Deserialize, Debug)]
pub struct Status {
    /// Status, "enabled" or "disabled"
    pub status: String,
}

/// Version Struct
#[derive(Deserialize, Debug)]
pub struct Version {
    /// Version
    pub version: u32,
}

/// Versions Struct
#[derive(Deserialize, Debug)]
pub struct Versions {
    /// Is there an update available for Pi-hole core
    pub core_update: bool,
    /// Is there an update available for Pi-hole web
    pub web_update: bool,
    /// Is there an update available for Pi-hole FTL
    #[serde(rename = "FTL_update")]
    pub ftl_update: bool,
    /// Current Pi-hole core version
    pub core_current: String,
    /// Current Pi-hole web version
    pub web_current: String,
    /// Current Pi-hole FTL version
    #[serde(rename = "FTL_current")]
    pub ftl_current: String,
    /// Latest Pi-hole core version
    pub core_latest: String,
    /// Latest Pi-hole web version
    pub web_latest: String,
    /// Latest Pi-hole FTL version
    #[serde(rename = "FTL_latest")]
    pub ftl_latest: String,
    /// Current Pi-hole core branch
    pub core_branch: String,
    /// Current Pi-hole web branch
    pub web_branch: String,
    /// Current Pi-hole FTL branch
    #[serde(rename = "FTL_branch")]
    pub ftl_branch: String,
}

/// Cache Info Struct
#[derive(Deserialize, Debug)]
pub struct CacheInfo {
    /// Cache size
    #[serde(rename = "cache-size")]
    pub cache_size: u64,

    /// Number of evicted cache entries
    #[serde(rename = "cache-live-freed")]
    pub cache_live_freed: u64,

    /// Number of cache entries inserted
    #[serde(rename = "cache-inserted")]
    pub cache_inserted: u64,
}

/// Client Name Struct
#[derive(Deserialize, Debug)]
pub struct ClientName {
    /// Client name
    pub name: String,

    /// Client IP
    pub ip: IpAddr,
}

/// Network Client Struct
#[derive(Deserialize, Debug)]
pub struct NetworkClient {
    /// Client ID
    pub id: u64,

    /// IP addresses of client
    pub ip: Vec<IpAddr>,

    /// Hardware address
    pub hwaddr: String,

    /// Interface
    pub interface: String,

    /// Hostname
    pub name: Vec<String>,

    /// Time first seen
    #[serde(rename = "firstSeen")]
    pub first_seen: u64,

    /// Time of last query
    #[serde(rename = "lastQuery")]
    pub last_query: u64,

    /// Number of queries
    #[serde(rename = "numQueries")]
    pub num_queries: u64,

    /// MAC Vendor
    #[serde(rename = "macVendor")]
    pub mac_vendor: String,
}

/// Network Struct
#[derive(Deserialize, Debug)]
pub struct Network {
    /// List of network clients
    pub network: Vec<NetworkClient>,
}

/// List Modification Response Struct
#[derive(Deserialize, Debug)]
pub struct ListModificationResponse {
    /// If request was successful
    pub success: bool,
    /// Optional message about request
    pub message: Option<String>,
}

/// Pi Hole API Struct
pub struct PiHoleAPI {
    /// Pi Hole host
    host: String,

    /// Optional API key
    api_key: Option<String>,
}

impl PiHoleAPI {
    /// Creates a new Pi Hole API instance.
    /// `host` must begin with the protocol e.g. http:// or https://
    pub fn new(host: String, api_key: Option<String>) -> Self {
        Self { host, api_key }
    }

    pub fn set_api_key(&mut self, api_key: String) {
        self.api_key = Some(api_key);
    }

    fn simple_json_request<T>(&self, path_query: &str) -> Result<T, errors::APIError>
    where
        T: DeserializeOwned,
    {
        let response = reqwest::blocking::get(&format!("{}{}", self.host, path_query))?;
        Ok(response.json()?)
    }

    fn authenticated_json_request<T>(&self, path_query: &str) -> Result<T, errors::APIError>
    where
        T: DeserializeOwned,
    {
        if self.api_key.is_none() {
            return Err(errors::APIError::MissingAPIKey);
        }

        let joining_char = if path_query.contains('?') { '&' } else { '?' };
        let auth_path_query = format!(
            "{}{}{}auth={}",
            self.host,
            path_query,
            joining_char,
            self.api_key.as_ref().unwrap()
        );
        let response = reqwest::blocking::get(&auth_path_query)?;
        println!("{:?}", reqwest::blocking::get(&auth_path_query)?.text()?);
        Ok(response.json()?)
    }

    /// Get statistics in a raw format (no number format)
    pub fn get_summary_raw(&self) -> Result<SummaryRaw, errors::APIError> {
        self.simple_json_request("/admin/api.php?summaryRaw")
    }

    /// Get statistics in a formatted style
    pub fn get_summary(&self) -> Result<Summary, errors::APIError> {
        self.simple_json_request("/admin/api.php?summary")
    }

    /// Get statistics on the number of domains and ads for each 10 minute period
    pub fn get_over_time_data_10_mins(&self) -> Result<OverTimeData, errors::APIError> {
        self.simple_json_request("/admin/api.php?overTimeData10mins")
    }

    /// Get the top domains and ads and the number of queries for each. Limit the number of items with `count`.
    /// API key required.
    pub fn get_top_items(&self, count: Option<u32>) -> Result<TopItems, errors::APIError> {
        self.authenticated_json_request(&format!("/admin/api.php?topItems={}", count.unwrap_or(10)))
    }

    /// Get the top clients and the number of queries for each. Limit the number of items with `count`.
    /// API key required.
    pub fn get_top_clients(&self, count: Option<u32>) -> Result<TopClients, errors::APIError> {
        self.authenticated_json_request(&format!(
            "/admin/api.php?topClients={}",
            count.unwrap_or(10)
        ))
    }

    /// Get the top clients blocked and the number of queries for each. Limit the number of items with `count`.
    /// API key required.
    pub fn get_top_clients_blocked(
        &self,
        count: Option<u32>,
    ) -> Result<TopClientsBlocked, errors::APIError> {
        self.authenticated_json_request(&format!(
            "/admin/api.php?topClientsBlocked={}",
            count.unwrap_or(10)
        ))
    }

    /// Get the number of queries forwarded and the target.
    /// API key required.
    pub fn get_forward_destinations(&self) -> Result<ForwardDestinations, errors::APIError> {
        self.authenticated_json_request("/admin/api.php?getForwardDestinations")
    }

    /// Get the number of queries per type.
    /// API key required.
    pub fn get_query_types(&self) -> Result<QueryTypes, errors::APIError> {
        self.authenticated_json_request("/admin/api.php?getQueryTypes")
    }

    /// Get all DNS query data. Limit the number of items with `count`.
    /// API key required.
    pub fn get_all_queries(&self, count: u32) -> Result<AllQueries, errors::APIError> {
        let raw_data: HashMap<String, Vec<Vec<String>>> =
            self.authenticated_json_request(&format!("/admin/api.php?getAllQueries={}", count))?;

        // Convert the queries from a list into a more useful Query struct
        let data = AllQueries {
            data: raw_data
                .get("data")
                .unwrap()
                .iter()
                .map(|raw_query| Query {
                    timestring: raw_query[0].clone(),
                    query_type: raw_query[1].clone(),
                    domain: raw_query[2].clone(),
                    client: raw_query[3].clone(),
                    answer_type: raw_query[4].clone(),
                })
                .collect(),
        };
        Ok(data)
    }

    /// Enable the Pi-Hole.
    /// API key required.
    pub fn enable(&self) -> Result<Status, errors::APIError> {
        self.authenticated_json_request("/admin/api.php?enable")
    }

    /// Disable the Pi-Hole for `seconds` seconds.
    /// API key required.
    pub fn disable(&self, seconds: u64) -> Result<Status, errors::APIError> {
        self.authenticated_json_request(&format!("/admin/api.php?disable={}", seconds))
    }

    /// Get the Pi-Hole version.
    pub fn get_version(&self) -> Result<u32, errors::APIError> {
        let raw_version: Version = self.simple_json_request("/admin/api.php?version")?;
        Ok(raw_version.version)
    }

    /// Get the detailed Pi-Hole versions for core, FTL and web interface.
    pub fn get_versions(&self) -> Result<Versions, errors::APIError> {
        self.simple_json_request("/admin/api.php?versions")
    }

    /// Get statistics about the DNS cache.
    /// API key required.
    pub fn get_cache_info(&self) -> Result<CacheInfo, errors::APIError> {
        let mut raw_data: HashMap<String, CacheInfo> =
            self.authenticated_json_request("/admin/api.php?getCacheInfo")?;
        Ok(raw_data.remove("cacheinfo").expect("Missing cache info"))
    }

    /// Get hostname and IP for hosts
    /// API key required.
    pub fn get_client_names(&self) -> Result<Vec<ClientName>, errors::APIError> {
        let mut raw_data: HashMap<String, Vec<ClientName>> =
            self.authenticated_json_request("/admin/api.php?getClientNames")?;
        Ok(raw_data
            .remove("clients")
            .expect("Missing clients attribute"))
    }

    /// Get queries by client over time. Maps timestamp to the number of queries by clients.
    /// Order of clients in the Vector is the same as for get_client_names
    /// API key required.
    pub fn get_over_time_data_clients(
        &self,
    ) -> Result<HashMap<String, Vec<u64>>, errors::APIError> {
        let mut raw_data: HashMap<String, FakeHashMap<String, Vec<u64>>> =
            self.authenticated_json_request("/admin/api.php?overTimeDataClients")?;

        Ok(raw_data
            .remove("over_time")
            .expect("Missing over_time attribute")
            .into())
    }

    /// Get information about network clients.
    /// API key required.
    pub fn get_network(&self) -> Result<Network, errors::APIError> {
        self.authenticated_json_request("/admin/api_db.php?network")
    }

    /// Get the total number of queries received.
    /// API key required.
    pub fn get_queries_count(&self) -> Result<u64, errors::APIError> {
        let raw_data: HashMap<String, u64> =
            self.authenticated_json_request("/admin/api_db.php?getQueriesCount")?;
        Ok(*raw_data.get("count").expect("Missing count attribute"))
    }

    /// Add domains to a custom white/blacklist.
    /// Acceptable lists are: `white`, `black`, `white_regex`, `black_regex`, `white_wild`, `black_wild`, `audit`.
    /// API key required.
    pub fn add(
        &self,
        domains: &[&str],
        list: &str,
    ) -> Result<ListModificationResponse, errors::APIError> {
        self.modify_list(domains, list, "add")
    }

    /// Remove domains to a custom white/blacklist.
    /// Acceptable lists are: `white`, `black`, `white_regex`, `black_regex`, `white_wild`, `black_wild`, `audit`.
    /// API key required.
    pub fn remove(
        &self,
        domains: &[&str],
        list: &str,
    ) -> Result<ListModificationResponse, errors::APIError> {
        self.modify_list(domains, list, "sub")
    }

    /// Perform a custom white/blacklist action ("add" or "sub")
    fn modify_list(
        &self,
        domains: &[&str],
        list: &str,
        action: &str,
    ) -> Result<ListModificationResponse, errors::APIError> {
        let url = format!(
            "{}/admin/api.php?{}={}&list={}&auth={}",
            self.host,
            action,
            domains.join(" "),
            list,
            self.api_key.as_ref().unwrap_or(&"".to_string())
        );

        let response_text = reqwest::blocking::get(&url)?.text()?;
        if response_text.starts_with("Invalid list") {
            Err(errors::APIError::InvalidList)
        } else {
            match serde_json::from_str::<ListModificationResponse>(&response_text) {
                Ok(response) => Ok(response),
                Err(error) => Err(error.into()),
            }
        }
    }
}

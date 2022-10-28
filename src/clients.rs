// Generated file, last update was: 2022-10-28 13:31
#[allow(non_camel_case_types)]
#[derive(Clone, Debug)]
pub enum ClientVersion {
    Bittorrent_7_10_1_43917,
}

impl crate::Client {
    /// Build and return the client drom the given key
    pub fn from(client_version: ClientVersion) -> crate::Client {
        match client_version {
            Bittorrent_7_10_1_43917 => crate::Client {
                name: String::from("bittorrent-7.10.1_43917"),
                key_algorithm: crate::algorithm::Algorithm::Hash,
                key_uppercase: Some(true),
                key_refresh_on: crate::RefreshInterval::TimedOrAfterStartedAnnounce,
                peer_pattern: String::from("-BT71000(«)[-ÿ]{10}"),
                uppercase_encoded_hex: false,
                num_want: 200, num_want_on_stop: 0,
                query: String::from("info_hash={infohash}&peer_id={peerid}&port={port}&uploaded={uploaded}&downloaded={downloaded}&left={left}&corrupt=0&key={key}&event={event}&numwant={numwant}&compact=1&no_peer_id=1"),
                encoding_exclusion_pattern: String::from("[A-Za-z0-9-]"),
                peer_url_encode: true,
                user_agent: String::from("BitTorrent/7100(255961997)(43917)"),
                connection: Some(String::from("Close")),
                accept_language: String::from(""),
                ..crate::Client::default()
            },
        }
    }
}

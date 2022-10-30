use url::form_urlencoded::byte_serialize;

mod algorithm;
pub mod clients;

const PEER_ID_LENGTH: usize = 20;
const KEY_LENGTH: usize = 8;

#[derive(Debug, Clone)]
pub enum RefreshInterval {
    Never,
    TimedOrAfterStartedAnnounce,
    TorrentVolatile,
    TorrentPersistent,
}

#[derive(Debug, Clone)]
pub struct Client {
    pub name: String,
    pub key :String,
    pub peer_id: String,
    pub key_refresh_every: Option<u16>,
    pub query: String,
    //request_headers: HashMap<String, String>, //HashMap<&str, i32> = [("Norway", 100), ("Denmark", 50), ("Iceland", 10)]
    pub user_agent: String,
    pub accept:String,
    pub accept_encoding: String,
    pub accept_language: String, //for some version of µTorrent
    pub connection:Option<String>,
    /// Optional. Number of peers that the client would like to receive from the tracker. This value is permitted to be zero. If omitted, typically defaults to 50 peers.
    pub num_want: u16,
    pub num_want_on_stop: u16,

    //Client configuration
    //----------- algorithms
    key_algorithm: algorithm::Algorithm, //length=8
    //key_length: u8, //key algorithm, key length is always 8
    key_pattern: String,
    key_refresh_on: RefreshInterval,
    key_uppercase: Option<bool>,
    peer_url_encode: bool,
    //----------- peer ID
    peer_algorithm: algorithm::Algorithm,
    ///for REGEX method, for RANDOM_POOL_WITH_CHECKSUM: list pf available chars, the base is the length of the string
    peer_pattern: String,
    /// for RANDOM_POOL_WITH_CHECKSUM
    peer_prefix: String,
    peer_refresh_on: RefreshInterval,
    //----------- URL encoder 
    encoding_exclusion_pattern: String,
    /// if the encoded hex string should be in upper case or no
    uppercase_encoded_hex: bool,
}

impl Client {
    pub fn default() -> Self { Client {
        //client configuration
        //key generator default values
        key_algorithm: algorithm::Algorithm::Hash,
        key_pattern:String::new(),
        key_uppercase: None,
        key_refresh_on: RefreshInterval::TimedOrAfterStartedAnnounce,
        key_refresh_every: None,
        //peer ID generator
        peer_algorithm: algorithm::Algorithm::Regex,
        peer_pattern: String::new(), peer_prefix:String::new(),
        peer_refresh_on: RefreshInterval::Never,
        peer_url_encode: false,
        //URL encoder
        encoding_exclusion_pattern: r"[A-Za-z0-9-]".to_owned(),
        uppercase_encoded_hex: false,
        //misc
        num_want: 200,
        num_want_on_stop: 0,
        //query headers
        query: "info_hash={infohash}&peer_id={peerid}&port={port}&uploaded={uploaded}&downloaded={downloaded}&left={left}&corrupt=0&key={key}&event={event}&numwant={numwant}&compact=1&no_peer_id=1".to_owned(),
        user_agent: String::with_capacity(64), //must be defined
        accept: String::new(),
        accept_encoding: String::from("gzip"),
        accept_language: String::with_capacity(5),
        connection: Some(String::from("Close")),
        key: String::new(),
        peer_id: String::new(),
        name: String::from("INVALID"),
    }}

    /// Returns the query to append to your announce URL. Variables are:
    /// * `{infohash}`:
    /// * `{peerid}`:
    /// * `{port}`: torrent port
    /// * `{uploaded}`: uploaded data in bytes
    /// * `{downloaded}`: downloaded data in bytes
    /// * `{left}`: remaining data to download in bytes
    /// * `{key}`:
    /// * `{event}`:
    /// * `{numwant}`:
    /// * `{os}` and `{java}` for Vuze
    /// 
    /// Returns: (URL, Vec<(Header name, Header value)>)
    pub fn get_query(&self) -> (String, Vec<(String,String)>) {
        let mut headers: Vec<(String,String)> = Vec::with_capacity(4);
        if !self.user_agent.is_empty() {headers.push((String::from("User-Agent"), self.user_agent.clone()));}
        if !self.accept.is_empty() {headers.push((String::from("Accept"), self.accept.clone()));}
        if !self.accept_encoding.is_empty() {headers.push((String::from("Accept-Encoding"), self.accept_encoding.clone()));}
        if !self.accept_language.is_empty() {headers.push((String::from("Accept-Language"), self.accept_language.clone()));}
        (self.query.clone(), headers)
    }

    /// Generate the client key, and encode it for HTTP request
    pub fn generate_key(&mut self) {
        self.key = match &self.key_algorithm {
            algorithm::Algorithm::Hash => algorithm::hash(false, self.key_uppercase),
            algorithm::Algorithm::HashNoLeadingZero => algorithm::hash(true, self.key_uppercase),
            algorithm::Algorithm::DigitRangeTransformedToHexWithoutLeadingZeroes => algorithm::digit_range_transformed_to_hex_without_leading_zero(),
            algorithm::Algorithm::Regex => byte_serialize(&algorithm::regex(self.peer_pattern.clone()).as_bytes()[0..KEY_LENGTH]).collect(),
            _ => String::with_capacity(KEY_LENGTH),
        };
    }
    /// Generate the peer ID and encode it for HTTP request
    pub fn generate_peer_id(&mut self) {
        let hash = match &self.peer_algorithm {
            algorithm::Algorithm::Regex                  => algorithm::regex(self.peer_pattern.clone()), //replace \ otherwise the generator crashes
            algorithm::Algorithm::RandomPoolWithChecksum => algorithm::random_pool_with_checksum(&self.peer_prefix, &self.peer_pattern),
            _ => String::new()
        };
        self.peer_id = byte_serialize(&hash.as_bytes()[0..PEER_ID_LENGTH]).collect(); //take the first 20 charsencode it because weird chars
    }
}

#[cfg(test)]
mod tests {
    use crate::{Client, clients::ClientVersion};

    const CLIENT_VERSIONS: [ClientVersion; 62] = [
        ClientVersion::Bittorrent_7_10_1_43917, ClientVersion::Bittorrent_7_10_3_44359, ClientVersion::Bittorrent_7_10_3_44429,
        ClientVersion::Deluge_1_3_13, ClientVersion::Deluge_1_3_14, ClientVersion::Deluge_1_3_15, ClientVersion::Deluge_2_0_3,
        ClientVersion::Leap_2_6_0_1,
        ClientVersion::Rtorrent_0_9_6_0_13_6,
        ClientVersion::Transmission_2_82_14160, ClientVersion::Transmission_2_92_14714, ClientVersion::Transmission_2_93, ClientVersion::Transmission_2_94, ClientVersion::Transmission_3_00,
        ClientVersion::Utorrent_3_2_2_28500, ClientVersion::Utorrent_3_5_0_43916, ClientVersion::Utorrent_3_5_0_44090, ClientVersion::Utorrent_3_5_0_44294, ClientVersion::Utorrent_3_5_1_44332, ClientVersion::Utorrent_3_5_3_44358, ClientVersion::Utorrent_3_5_3_44428, ClientVersion::Utorrent_3_5_4_44498,
        ClientVersion::Vuze_5_7_5_0,
        //QBittorrent
        ClientVersion::Qbittorrent_3_3_1, ClientVersion::Qbittorrent_3_3_13, ClientVersion::Qbittorrent_3_3_14, ClientVersion::Qbittorrent_3_3_15, ClientVersion::Qbittorrent_3_3_16, ClientVersion::Qbittorrent_3_3_7,
        ClientVersion::Qbittorrent_4_0_0, ClientVersion::Qbittorrent_4_0_1, ClientVersion::Qbittorrent_4_0_2, ClientVersion::Qbittorrent_4_0_3, ClientVersion::Qbittorrent_4_0_4,
        ClientVersion::Qbittorrent_4_1_0, ClientVersion::Qbittorrent_4_1_1, ClientVersion::Qbittorrent_4_1_2, ClientVersion::Qbittorrent_4_1_3, ClientVersion::Qbittorrent_4_1_4, ClientVersion::Qbittorrent_4_1_5, ClientVersion::Qbittorrent_4_1_6, ClientVersion::Qbittorrent_4_1_7, ClientVersion::Qbittorrent_4_1_8, ClientVersion::Qbittorrent_4_1_9,
        ClientVersion::Qbittorrent_4_2_0, ClientVersion::Qbittorrent_4_2_1, ClientVersion::Qbittorrent_4_2_2, ClientVersion::Qbittorrent_4_2_3, ClientVersion::Qbittorrent_4_2_4, ClientVersion::Qbittorrent_4_2_5,
        ClientVersion::Qbittorrent_4_3_0_1, ClientVersion::Qbittorrent_4_3_0, ClientVersion::Qbittorrent_4_3_1, ClientVersion::Qbittorrent_4_3_2, ClientVersion::Qbittorrent_4_3_3, ClientVersion::Qbittorrent_4_3_4_1, ClientVersion::Qbittorrent_4_3_5, ClientVersion::Qbittorrent_4_3_6, ClientVersion::Qbittorrent_4_3_8, ClientVersion::Qbittorrent_4_3_9,
        ClientVersion::Qbittorrent_4_4_2, ClientVersion::Qbittorrent_4_4_3_1
    ];

    #[test]
    fn check_queries() {
        for cv in crate::tests::CLIENT_VERSIONS {
            let c = Client::from(cv);
            let q = c.query;
            assert!(q.contains("info_hash={infohash}"));
            assert!(q.contains("peer_id={peerid}"));
            assert!(q.contains("uploaded={uploaded}"));
            assert!(q.contains("downloaded={downloaded}"));
            assert!(q.contains("left={left}"));
            assert!(q.contains("key={key}"));
            assert!(q.contains("event={event}"));
            if !c.name.starts_with("rtorrent") {
                assert!(q.contains("numwant={numwant}"));
            }
            if q.contains("ipv6=") || q.contains("{ipv6}") {
                assert!(q.contains("ipv6={ipv6}"));
            }
            if q.contains("ip=") || q.contains("{ip}") {
                assert!(q.contains("ip={ip}"));
            }
            assert!(!q.contains("&&"));
            assert!(!q.starts_with('&'));
            assert!(!q.ends_with('&'));
        }
    }
}


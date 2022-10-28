mod algorithm;
mod clients;

const PEER_ID_LENGTH: usize = 20;
const KEY_LENGTH: u8 = 8;

pub enum RefreshInterval {
    Never,
    TimedOrAfterStartedAnnounce,
    TorrentVolatile,
    TorrentPersistent,
}

pub struct Client {
    pub name: String,
    pub key :String,
    pub peer_id: String,
    pub key_refresh_every: u16,
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
        key_refresh_every: 0,
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
}

// Generated file, last update was: 2022-10-30 12:46s
#[allow(non_camel_case_types)]
#[derive(Clone, Debug, strum_macros::EnumString)]
pub enum ClientVersion {
    Bittorrent_7_10_1_43917,
    Bittorrent_7_10_3_44359,
    Bittorrent_7_10_3_44429,
    Deluge_1_3_13,
    Deluge_1_3_14,
    Deluge_1_3_15,
    Deluge_2_0_3,
    Leap_2_6_0_1,
    Qbittorrent_3_3_1,
    Qbittorrent_3_3_13,
    Qbittorrent_3_3_14,
    Qbittorrent_3_3_15,
    Qbittorrent_3_3_16,
    Qbittorrent_3_3_7,
    Qbittorrent_4_0_0,
    Qbittorrent_4_0_1,
    Qbittorrent_4_0_2,
    Qbittorrent_4_0_3,
    Qbittorrent_4_0_4,
    Qbittorrent_4_1_0,
    Qbittorrent_4_1_1,
    Qbittorrent_4_1_2,
    Qbittorrent_4_1_3,
    Qbittorrent_4_1_4,
    Qbittorrent_4_1_5,
    Qbittorrent_4_1_6,
    Qbittorrent_4_1_7,
    Qbittorrent_4_1_8,
    Qbittorrent_4_1_9,
    Qbittorrent_4_2_0,
    Qbittorrent_4_2_1,
    Qbittorrent_4_2_2,
    Qbittorrent_4_2_3,
    Qbittorrent_4_2_4,
    Qbittorrent_4_2_5,
    Qbittorrent_4_3_0_1,
    Qbittorrent_4_3_0,
    Qbittorrent_4_3_1,
    Qbittorrent_4_3_2,
    Qbittorrent_4_3_3,
    Qbittorrent_4_3_4_1,
    Qbittorrent_4_3_5,
    Qbittorrent_4_3_6,
    Qbittorrent_4_3_8,
    Qbittorrent_4_3_9,
    Qbittorrent_4_4_2,
    Qbittorrent_4_4_3_1,
    Rtorrent_0_9_6_0_13_6,
    Transmission_2_82_14160,
    Transmission_2_92_14714,
    Transmission_2_93,
    Transmission_2_94,
    Transmission_3_00,
    Utorrent_3_2_2_28500,
    Utorrent_3_5_0_43916,
    Utorrent_3_5_0_44090,
    Utorrent_3_5_0_44294,
    Utorrent_3_5_1_44332,
    Utorrent_3_5_3_44358,
    Utorrent_3_5_3_44428,
    Utorrent_3_5_4_44498,
    Vuze_5_7_5_0,
}

impl crate::Client {
    /// Build and return the client drom the given key
    pub fn build(&mut self, client_version: ClientVersion) {
        match client_version {
            ClientVersion::Bittorrent_7_10_1_43917 => {
                self.name = String::from("bittorrent-7.10.1_43917");
                self.key_algorithm = crate::algorithm::Algorithm::Hash;
                self.key_uppercase = Some(true);
                self.key_refresh_on = crate::RefreshInterval::TimedOrAfterStartedAnnounce;
                self.key_refresh_every = Some(10);
                self.peer_algorithm = crate::algorithm::Algorithm::Regex;
                self.peer_pattern = String::from(r"b'-BT71000(\xc2\x8d\xc2\xab)[\x01-\xc3\xbf]{10}'");
                self.uppercase_encoded_hex = false;
                self.num_want = 200; self.num_want_on_stop = 0;
                self.query = String::from("info_hash={infohash}&peer_id={peerid}&port={port}&uploaded={uploaded}&downloaded={downloaded}&left={left}&corrupt=0&key={key}&event={event}&numwant={numwant}&compact=1&no_peer_id=1");
                self.encoding_exclusion_pattern = String::from(r"b'[A-Za-z0-9-]'");
                self.peer_url_encode = true;
                self.user_agent = String::from("BitTorrent/7100(255961997)(43917)");
                self.connection = Some(String::from("Close"));
                self.accept_language = String::from("");
            },
            ClientVersion::Bittorrent_7_10_3_44359 => {
                self.name = String::from("bittorrent-7.10.3_44359");
                self.key_algorithm = crate::algorithm::Algorithm::Hash;
                self.key_uppercase = Some(true);
                self.key_refresh_on = crate::RefreshInterval::TimedOrAfterStartedAnnounce;
                self.key_refresh_every = Some(10);
                self.peer_algorithm = crate::algorithm::Algorithm::Regex;
                self.peer_pattern = String::from(r"b'-BT7a3S-G(\xc2\xad)[\x01-\xc3\xbf]{10}'");
                self.uppercase_encoded_hex = false;
                self.num_want = 200; self.num_want_on_stop = 0;
                self.query = String::from("info_hash={infohash}&peer_id={peerid}&port={port}&uploaded={uploaded}&downloaded={downloaded}&left={left}&corrupt=0&key={key}&event={event}&numwant={numwant}&compact=1&no_peer_id=1");
                self.encoding_exclusion_pattern = String::from(r"b'[A-Za-z0-9-]'");
                self.peer_url_encode = true;
                self.user_agent = String::from("BitTorrent/7103(256355655)(44359)");
                self.connection = Some(String::from("Close"));
                self.accept_language = String::from("");
            },
            ClientVersion::Bittorrent_7_10_3_44429 => {
                self.name = String::from("bittorrent-7.10.3_44429");
                self.key_algorithm = crate::algorithm::Algorithm::Hash;
                self.key_uppercase = Some(true);
                self.key_refresh_on = crate::RefreshInterval::TimedOrAfterStartedAnnounce;
                self.key_refresh_every = Some(10);
                self.peer_algorithm = crate::algorithm::Algorithm::Regex;
                self.peer_pattern = String::from(r"b'-BT7a3S-(\xc2\x8d)(\xc2\xad)[\x01-\xc3\xbf]{10}'");
                self.uppercase_encoded_hex = false;
                self.num_want = 200; self.num_want_on_stop = 0;
                self.query = String::from("info_hash={infohash}&peer_id={peerid}&port={port}&uploaded={uploaded}&downloaded={downloaded}&left={left}&corrupt=0&key={key}&event={event}&numwant={numwant}&compact=1&no_peer_id=1");
                self.encoding_exclusion_pattern = String::from(r"b'[A-Za-z0-9-]'");
                self.peer_url_encode = true;
                self.user_agent = String::from("BitTorrent/7103(256355725)(44429)");
                self.connection = Some(String::from("Close"));
                self.accept_language = String::from("");
            },
            ClientVersion::Deluge_1_3_13 => {
                self.name = String::from("deluge-1.3.13");
                self.key_algorithm = crate::algorithm::Algorithm::HashNoLeadingZero;
                self.key_uppercase = Some(true);
                self.key_refresh_on = crate::RefreshInterval::TorrentPersistent;
                self.peer_algorithm = crate::algorithm::Algorithm::Regex;
                self.peer_pattern = String::from(r"b'-DE13D0-[A-Za-z0-9_~\\\\(\\\\)\\\\!\\\\.\\\\*-]{12}'");
                self.uppercase_encoded_hex = false;
                self.num_want = 200; self.num_want_on_stop = 0;
                self.query = String::from("info_hash={infohash}&peer_id={peerid}&port={port}&uploaded={uploaded}&downloaded={downloaded}&left={left}&corrupt=0&key={key}&event={event}&numwant={numwant}&compact=1&no_peer_id=1&supportcrypto=1&redundant=0");
                self.encoding_exclusion_pattern = String::from(r"b'[A-Za-z0-9_~\\\\(\\\\)\\\\!\\\\.\\\\*-]'");
                self.peer_url_encode = false;
                self.user_agent = String::from("Deluge 1.3.13");
                self.connection = Some(String::from("close"));
                self.accept_language = String::from("");
            },
            ClientVersion::Deluge_1_3_14 => {
                self.name = String::from("deluge-1.3.14");
                self.key_algorithm = crate::algorithm::Algorithm::HashNoLeadingZero;
                self.key_uppercase = Some(true);
                self.key_refresh_on = crate::RefreshInterval::TorrentPersistent;
                self.peer_algorithm = crate::algorithm::Algorithm::Regex;
                self.peer_pattern = String::from(r"b'-DE13E0-[A-Za-z0-9_~\\\\(\\\\)\\\\!\\\\.\\\\*-]{12}'");
                self.uppercase_encoded_hex = false;
                self.num_want = 200; self.num_want_on_stop = 0;
                self.query = String::from("info_hash={infohash}&peer_id={peerid}&port={port}&uploaded={uploaded}&downloaded={downloaded}&left={left}&corrupt=0&key={key}&event={event}&numwant={numwant}&compact=1&no_peer_id=1&supportcrypto=1&redundant=0");
                self.encoding_exclusion_pattern = String::from(r"b'[A-Za-z0-9_~\\\\(\\\\)\\\\!\\\\.\\\\*-]'");
                self.peer_url_encode = false;
                self.user_agent = String::from("Deluge 1.3.14");
                self.connection = Some(String::from("close"));
                self.accept_language = String::from("");
            },
            ClientVersion::Deluge_1_3_15 => {
                self.name = String::from("deluge-1.3.15");
                self.key_algorithm = crate::algorithm::Algorithm::HashNoLeadingZero;
                self.key_uppercase = Some(true);
                self.key_refresh_on = crate::RefreshInterval::TorrentPersistent;
                self.peer_algorithm = crate::algorithm::Algorithm::Regex;
                self.peer_pattern = String::from(r"b'-DE13F0-[A-Za-z0-9_~\\\\(\\\\)\\\\!\\\\.\\\\*-]{12}'");
                self.uppercase_encoded_hex = false;
                self.num_want = 200; self.num_want_on_stop = 0;
                self.query = String::from("info_hash={infohash}&peer_id={peerid}&port={port}&uploaded={uploaded}&downloaded={downloaded}&left={left}&corrupt=0&key={key}&event={event}&numwant={numwant}&compact=1&no_peer_id=1&supportcrypto=1&redundant=0");
                self.encoding_exclusion_pattern = String::from(r"b'[A-Za-z0-9_~\\\\(\\\\)\\\\!\\\\.\\\\*-]'");
                self.peer_url_encode = false;
                self.user_agent = String::from("Deluge 1.3.15");
                self.connection = Some(String::from("close"));
                self.accept_language = String::from("");
            },
            ClientVersion::Deluge_2_0_3 => {
                self.name = String::from("deluge-2.0.3");
                self.key_algorithm = crate::algorithm::Algorithm::HashNoLeadingZero;
                self.key_uppercase = Some(true);
                self.key_refresh_on = crate::RefreshInterval::TorrentPersistent;
                self.peer_algorithm = crate::algorithm::Algorithm::Regex;
                self.peer_pattern = String::from(r"b'-DE203s-[A-Za-z0-9_~\\\\(\\\\)\\\\!\\\\.\\\\*-]{12}'");
                self.uppercase_encoded_hex = false;
                self.num_want = 200; self.num_want_on_stop = 0;
                self.query = String::from("info_hash={infohash}&peer_id={peerid}&port={port}&uploaded={uploaded}&downloaded={downloaded}&left={left}&corrupt=0&key={key}&event={event}&numwant={numwant}&compact=1&no_peer_id=1&supportcrypto=1&redundant=0");
                self.encoding_exclusion_pattern = String::from(r"b'[A-Za-z0-9_~\\\\(\\\\)\\\\!\\\\.\\\\*-]'");
                self.peer_url_encode = false;
                self.user_agent = String::from("Deluge/2.0.3 libtorrent/2.0.5.0");
                self.connection = Some(String::from("close"));
                self.accept_language = String::from("");
            },
            ClientVersion::Leap_2_6_0_1 => {
                self.name = String::from("leap-2.6.0.1");
                self.key_algorithm = crate::algorithm::Algorithm::Hash;
                self.key_uppercase = Some(true);
                self.key_refresh_on = crate::RefreshInterval::TorrentPersistent;
                self.peer_algorithm = crate::algorithm::Algorithm::Regex;
                self.peer_pattern = String::from(r"b'-LT1100-[A-Za-z0-9_~\\\\(\\\\)\\\\!\\\\.\\\\*-]{12}'");
                self.uppercase_encoded_hex = false;
                self.num_want = 200; self.num_want_on_stop = 0;
                self.query = String::from("info_hash={infohash}&peer_id={peerid}&port={port}&uploaded={uploaded}&downloaded={downloaded}&left={left}&corrupt=0&key={key}&event={event}&numwant={numwant}&compact=1&no_peer_id=1&supportcrypto=1&redundant=0");
                self.encoding_exclusion_pattern = String::from(r"b'[A-Za-z0-9_~\\\\(\\\\)\\\\!\\\\.\\\\*-]'");
                self.peer_url_encode = false;
                self.user_agent = String::from("libtorrent_leap/1.1.1.0");
                self.connection = Some(String::from("close"));
                self.accept_language = String::from("");
            },
            ClientVersion::Qbittorrent_3_3_1 => {
                self.name = String::from("qbittorrent-3.3.1");
                self.key_algorithm = crate::algorithm::Algorithm::HashNoLeadingZero;
                self.key_uppercase = Some(true);
                self.key_refresh_on = crate::RefreshInterval::TorrentPersistent;
                self.peer_algorithm = crate::algorithm::Algorithm::Regex;
                self.peer_pattern = String::from(r"b'-qB3310-[A-Za-z0-9_~\\\\(\\\\)\\\\!\\\\.\\\\*-]{12}'");
                self.uppercase_encoded_hex = false;
                self.num_want = 200; self.num_want_on_stop = 0;
                self.query = String::from("info_hash={infohash}&peer_id={peerid}&port={port}&uploaded={uploaded}&downloaded={downloaded}&left={left}&corrupt=0&key={key}&event={event}&numwant={numwant}&compact=1&no_peer_id=1&supportcrypto=1&redundant=0");
                self.encoding_exclusion_pattern = String::from(r"b'[A-Za-z0-9_~\\\\(\\\\)\\\\!\\\\.\\\\*-]'");
                self.peer_url_encode = false;
                self.user_agent = String::from("qBittorrent v3.3.1");
                self.connection = Some(String::from("close"));
                self.accept_language = String::from("");
            },
            ClientVersion::Qbittorrent_3_3_13 => {
                self.name = String::from("qbittorrent-3.3.13");
                self.key_algorithm = crate::algorithm::Algorithm::HashNoLeadingZero;
                self.key_uppercase = Some(true);
                self.key_refresh_on = crate::RefreshInterval::TorrentPersistent;
                self.peer_algorithm = crate::algorithm::Algorithm::Regex;
                self.peer_pattern = String::from(r"b'-qB33D0-[A-Za-z0-9_~\\\\(\\\\)\\\\!\\\\.\\\\*-]{12}'");
                self.uppercase_encoded_hex = false;
                self.num_want = 200; self.num_want_on_stop = 0;
                self.query = String::from("info_hash={infohash}&peer_id={peerid}&port={port}&uploaded={uploaded}&downloaded={downloaded}&left={left}&corrupt=0&key={key}&event={event}&numwant={numwant}&compact=1&no_peer_id=1&supportcrypto=1&redundant=0");
                self.encoding_exclusion_pattern = String::from(r"b'[A-Za-z0-9_~\\\\(\\\\)\\\\!\\\\.\\\\*-]'");
                self.peer_url_encode = false;
                self.user_agent = String::from("qBittorrent/3.3.13");
                self.connection = Some(String::from("close"));
                self.accept_language = String::from("");
            },
            ClientVersion::Qbittorrent_3_3_14 => {
                self.name = String::from("qbittorrent-3.3.14");
                self.key_algorithm = crate::algorithm::Algorithm::HashNoLeadingZero;
                self.key_uppercase = Some(true);
                self.key_refresh_on = crate::RefreshInterval::TorrentPersistent;
                self.peer_algorithm = crate::algorithm::Algorithm::Regex;
                self.peer_pattern = String::from(r"b'-qB33E0-[A-Za-z0-9_~\\\\(\\\\)\\\\!\\\\.\\\\*-]{12}'");
                self.uppercase_encoded_hex = false;
                self.num_want = 200; self.num_want_on_stop = 0;
                self.query = String::from("info_hash={infohash}&peer_id={peerid}&port={port}&uploaded={uploaded}&downloaded={downloaded}&left={left}&corrupt=0&key={key}&event={event}&numwant={numwant}&compact=1&no_peer_id=1&supportcrypto=1&redundant=0");
                self.encoding_exclusion_pattern = String::from(r"b'[A-Za-z0-9_~\\\\(\\\\)\\\\!\\\\.\\\\*-]'");
                self.peer_url_encode = false;
                self.user_agent = String::from("qBittorrent/3.3.14");
                self.connection = Some(String::from("close"));
                self.accept_language = String::from("");
            },
            ClientVersion::Qbittorrent_3_3_15 => {
                self.name = String::from("qbittorrent-3.3.15");
                self.key_algorithm = crate::algorithm::Algorithm::HashNoLeadingZero;
                self.key_uppercase = Some(true);
                self.key_refresh_on = crate::RefreshInterval::TorrentPersistent;
                self.peer_algorithm = crate::algorithm::Algorithm::Regex;
                self.peer_pattern = String::from(r"b'-qB33F0-[A-Za-z0-9_~\\\\(\\\\)\\\\!\\\\.\\\\*-]{12}'");
                self.uppercase_encoded_hex = false;
                self.num_want = 200; self.num_want_on_stop = 0;
                self.query = String::from("info_hash={infohash}&peer_id={peerid}&port={port}&uploaded={uploaded}&downloaded={downloaded}&left={left}&corrupt=0&key={key}&event={event}&numwant={numwant}&compact=1&no_peer_id=1&supportcrypto=1&redundant=0");
                self.encoding_exclusion_pattern = String::from(r"b'[A-Za-z0-9_~\\\\(\\\\)\\\\!\\\\.\\\\*-]'");
                self.peer_url_encode = false;
                self.user_agent = String::from("qBittorrent/3.3.15");
                self.connection = Some(String::from("close"));
                self.accept_language = String::from("");
            },
            ClientVersion::Qbittorrent_3_3_16 => {
                self.name = String::from("qbittorrent-3.3.16");
                self.key_algorithm = crate::algorithm::Algorithm::HashNoLeadingZero;
                self.key_uppercase = Some(true);
                self.key_refresh_on = crate::RefreshInterval::TorrentPersistent;
                self.peer_algorithm = crate::algorithm::Algorithm::Regex;
                self.peer_pattern = String::from(r"b'-qB33G0-[A-Za-z0-9_~\\\\(\\\\)\\\\!\\\\.\\\\*-]{12}'");
                self.uppercase_encoded_hex = false;
                self.num_want = 200; self.num_want_on_stop = 0;
                self.query = String::from("info_hash={infohash}&peer_id={peerid}&port={port}&uploaded={uploaded}&downloaded={downloaded}&left={left}&corrupt=0&key={key}&event={event}&numwant={numwant}&compact=1&no_peer_id=1&supportcrypto=1&redundant=0");
                self.encoding_exclusion_pattern = String::from(r"b'[A-Za-z0-9_~\\\\(\\\\)\\\\!\\\\.\\\\*-]'");
                self.peer_url_encode = false;
                self.user_agent = String::from("qBittorrent/3.3.16");
                self.connection = Some(String::from("close"));
                self.accept_language = String::from("");
            },
            ClientVersion::Qbittorrent_3_3_7 => {
                self.name = String::from("qbittorrent-3.3.7");
                self.key_algorithm = crate::algorithm::Algorithm::HashNoLeadingZero;
                self.key_uppercase = Some(true);
                self.key_refresh_on = crate::RefreshInterval::TorrentPersistent;
                self.peer_algorithm = crate::algorithm::Algorithm::Regex;
                self.peer_pattern = String::from(r"b'-qB3310-[A-Za-z0-9_~\\\\(\\\\)\\\\!\\\\.\\\\*-]{12}'");
                self.uppercase_encoded_hex = false;
                self.num_want = 200; self.num_want_on_stop = 0;
                self.query = String::from("info_hash={infohash}&peer_id={peerid}&port={port}&uploaded={uploaded}&downloaded={downloaded}&left={left}&corrupt=0&key={key}&event={event}&numwant={numwant}&compact=1&no_peer_id=1&supportcrypto=1&redundant=0");
                self.encoding_exclusion_pattern = String::from(r"b'[A-Za-z0-9_~\\\\(\\\\)\\\\!\\\\.\\\\*-]'");
                self.peer_url_encode = false;
                self.user_agent = String::from("qBittorrent v3.3.7");
                self.connection = Some(String::from("close"));
                self.accept_language = String::from("");
            },
            ClientVersion::Qbittorrent_4_0_0 => {
                self.name = String::from("qbittorrent-4.0.0");
                self.key_algorithm = crate::algorithm::Algorithm::HashNoLeadingZero;
                self.key_uppercase = Some(true);
                self.key_refresh_on = crate::RefreshInterval::TorrentPersistent;
                self.peer_algorithm = crate::algorithm::Algorithm::Regex;
                self.peer_pattern = String::from(r"b'-qB4000-[A-Za-z0-9_~\\\\(\\\\)\\\\!\\\\.\\\\*-]{12}'");
                self.uppercase_encoded_hex = false;
                self.num_want = 200; self.num_want_on_stop = 0;
                self.query = String::from("info_hash={infohash}&peer_id={peerid}&port={port}&uploaded={uploaded}&downloaded={downloaded}&left={left}&corrupt=0&key={key}&event={event}&numwant={numwant}&compact=1&no_peer_id=1&supportcrypto=1&redundant=0");
                self.encoding_exclusion_pattern = String::from(r"b'[A-Za-z0-9_~\\\\(\\\\)\\\\!\\\\.\\\\*-]'");
                self.peer_url_encode = false;
                self.user_agent = String::from("qBittorrent/4.0.0");
                self.connection = Some(String::from("close"));
                self.accept_language = String::from("");
            },
            ClientVersion::Qbittorrent_4_0_1 => {
                self.name = String::from("qbittorrent-4.0.1");
                self.key_algorithm = crate::algorithm::Algorithm::HashNoLeadingZero;
                self.key_uppercase = Some(true);
                self.key_refresh_on = crate::RefreshInterval::TorrentPersistent;
                self.peer_algorithm = crate::algorithm::Algorithm::Regex;
                self.peer_pattern = String::from(r"b'-qB4010-[A-Za-z0-9_~\\\\(\\\\)\\\\!\\\\.\\\\*-]{12}'");
                self.uppercase_encoded_hex = false;
                self.num_want = 200; self.num_want_on_stop = 0;
                self.query = String::from("info_hash={infohash}&peer_id={peerid}&port={port}&uploaded={uploaded}&downloaded={downloaded}&left={left}&corrupt=0&key={key}&event={event}&numwant={numwant}&compact=1&no_peer_id=1&supportcrypto=1&redundant=0");
                self.encoding_exclusion_pattern = String::from(r"b'[A-Za-z0-9_~\\\\(\\\\)\\\\!\\\\.\\\\*-]'");
                self.peer_url_encode = false;
                self.user_agent = String::from("qBittorrent/4.0.1");
                self.connection = Some(String::from("close"));
                self.accept_language = String::from("");
            },
            ClientVersion::Qbittorrent_4_0_2 => {
                self.name = String::from("qbittorrent-4.0.2");
                self.key_algorithm = crate::algorithm::Algorithm::HashNoLeadingZero;
                self.key_uppercase = Some(true);
                self.key_refresh_on = crate::RefreshInterval::TorrentPersistent;
                self.peer_algorithm = crate::algorithm::Algorithm::Regex;
                self.peer_pattern = String::from(r"b'-qB4020-[A-Za-z0-9_~\\\\(\\\\)\\\\!\\\\.\\\\*-]{12}'");
                self.uppercase_encoded_hex = false;
                self.num_want = 200; self.num_want_on_stop = 0;
                self.query = String::from("info_hash={infohash}&peer_id={peerid}&port={port}&uploaded={uploaded}&downloaded={downloaded}&left={left}&corrupt=0&key={key}&event={event}&numwant={numwant}&compact=1&no_peer_id=1&supportcrypto=1&redundant=0");
                self.encoding_exclusion_pattern = String::from(r"b'[A-Za-z0-9_~\\\\(\\\\)\\\\!\\\\.\\\\*-]'");
                self.peer_url_encode = false;
                self.user_agent = String::from("qBittorrent/4.0.2");
                self.connection = Some(String::from("close"));
                self.accept_language = String::from("");
            },
            ClientVersion::Qbittorrent_4_0_3 => {
                self.name = String::from("qbittorrent-4.0.3");
                self.key_algorithm = crate::algorithm::Algorithm::HashNoLeadingZero;
                self.key_uppercase = Some(true);
                self.key_refresh_on = crate::RefreshInterval::TorrentPersistent;
                self.peer_algorithm = crate::algorithm::Algorithm::Regex;
                self.peer_pattern = String::from(r"b'-qB4030-[A-Za-z0-9_~\\\\(\\\\)\\\\!\\\\.\\\\*-]{12}'");
                self.uppercase_encoded_hex = false;
                self.num_want = 200; self.num_want_on_stop = 0;
                self.query = String::from("info_hash={infohash}&peer_id={peerid}&port={port}&uploaded={uploaded}&downloaded={downloaded}&left={left}&corrupt=0&key={key}&event={event}&numwant={numwant}&compact=1&no_peer_id=1&supportcrypto=1&redundant=0");
                self.encoding_exclusion_pattern = String::from(r"b'[A-Za-z0-9_~\\\\(\\\\)\\\\!\\\\.\\\\*-]'");
                self.peer_url_encode = false;
                self.user_agent = String::from("qBittorrent/4.0.3");
                self.connection = Some(String::from("close"));
                self.accept_language = String::from("");
            },
            ClientVersion::Qbittorrent_4_0_4 => {
                self.name = String::from("qbittorrent-4.0.4");
                self.key_algorithm = crate::algorithm::Algorithm::HashNoLeadingZero;
                self.key_uppercase = Some(true);
                self.key_refresh_on = crate::RefreshInterval::TorrentPersistent;
                self.peer_algorithm = crate::algorithm::Algorithm::Regex;
                self.peer_pattern = String::from(r"b'-qB4040-[A-Za-z0-9_~\\\\(\\\\)\\\\!\\\\.\\\\*-]{12}'");
                self.uppercase_encoded_hex = false;
                self.num_want = 200; self.num_want_on_stop = 0;
                self.query = String::from("info_hash={infohash}&peer_id={peerid}&port={port}&uploaded={uploaded}&downloaded={downloaded}&left={left}&corrupt=0&key={key}&event={event}&numwant={numwant}&compact=1&no_peer_id=1&supportcrypto=1&redundant=0");
                self.encoding_exclusion_pattern = String::from(r"b'[A-Za-z0-9_~\\\\(\\\\)\\\\!\\\\.\\\\*-]'");
                self.peer_url_encode = false;
                self.user_agent = String::from("qBittorrent/4.0.4");
                self.connection = Some(String::from("close"));
                self.accept_language = String::from("");
            },
            ClientVersion::Qbittorrent_4_1_0 => {
                self.name = String::from("qbittorrent-4.1.0");
                self.key_algorithm = crate::algorithm::Algorithm::HashNoLeadingZero;
                self.key_uppercase = Some(true);
                self.key_refresh_on = crate::RefreshInterval::TorrentPersistent;
                self.peer_algorithm = crate::algorithm::Algorithm::Regex;
                self.peer_pattern = String::from(r"b'-qB4100-[A-Za-z0-9_~\\\\(\\\\)\\\\!\\\\.\\\\*-]{12}'");
                self.uppercase_encoded_hex = false;
                self.num_want = 200; self.num_want_on_stop = 0;
                self.query = String::from("info_hash={infohash}&peer_id={peerid}&port={port}&uploaded={uploaded}&downloaded={downloaded}&left={left}&corrupt=0&key={key}&event={event}&numwant={numwant}&compact=1&no_peer_id=1&supportcrypto=1&redundant=0");
                self.encoding_exclusion_pattern = String::from(r"b'[A-Za-z0-9_~\\\\(\\\\)\\\\!\\\\.\\\\*-]'");
                self.peer_url_encode = false;
                self.user_agent = String::from("qBittorrent/4.1.0");
                self.connection = Some(String::from("close"));
                self.accept_language = String::from("");
            },
            ClientVersion::Qbittorrent_4_1_1 => {
                self.name = String::from("qbittorrent-4.1.1");
                self.key_algorithm = crate::algorithm::Algorithm::HashNoLeadingZero;
                self.key_uppercase = Some(true);
                self.key_refresh_on = crate::RefreshInterval::TorrentPersistent;
                self.peer_algorithm = crate::algorithm::Algorithm::Regex;
                self.peer_pattern = String::from(r"b'-qB4110-[A-Za-z0-9_~\\\\(\\\\)\\\\!\\\\.\\\\*-]{12}'");
                self.uppercase_encoded_hex = false;
                self.num_want = 200; self.num_want_on_stop = 0;
                self.query = String::from("info_hash={infohash}&peer_id={peerid}&port={port}&uploaded={uploaded}&downloaded={downloaded}&left={left}&corrupt=0&key={key}&event={event}&numwant={numwant}&compact=1&no_peer_id=1&supportcrypto=1&redundant=0");
                self.encoding_exclusion_pattern = String::from(r"b'[A-Za-z0-9_~\\\\(\\\\)\\\\!\\\\.\\\\*-]'");
                self.peer_url_encode = false;
                self.user_agent = String::from("qBittorrent/4.1.1");
                self.connection = Some(String::from("close"));
                self.accept_language = String::from("");
            },
            ClientVersion::Qbittorrent_4_1_2 => {
                self.name = String::from("qbittorrent-4.1.2");
                self.key_algorithm = crate::algorithm::Algorithm::HashNoLeadingZero;
                self.key_uppercase = Some(true);
                self.key_refresh_on = crate::RefreshInterval::TorrentPersistent;
                self.peer_algorithm = crate::algorithm::Algorithm::Regex;
                self.peer_pattern = String::from(r"b'-qB4120-[A-Za-z0-9_~\\\\(\\\\)\\\\!\\\\.\\\\*-]{12}'");
                self.uppercase_encoded_hex = false;
                self.num_want = 200; self.num_want_on_stop = 0;
                self.query = String::from("info_hash={infohash}&peer_id={peerid}&port={port}&uploaded={uploaded}&downloaded={downloaded}&left={left}&corrupt=0&key={key}&event={event}&numwant={numwant}&compact=1&no_peer_id=1&supportcrypto=1&redundant=0");
                self.encoding_exclusion_pattern = String::from(r"b'[A-Za-z0-9_~\\\\(\\\\)\\\\!\\\\.\\\\*-]'");
                self.peer_url_encode = false;
                self.user_agent = String::from("qBittorrent/4.1.2");
                self.connection = Some(String::from("close"));
                self.accept_language = String::from("");
            },
            ClientVersion::Qbittorrent_4_1_3 => {
                self.name = String::from("qbittorrent-4.1.3");
                self.key_algorithm = crate::algorithm::Algorithm::HashNoLeadingZero;
                self.key_uppercase = Some(true);
                self.key_refresh_on = crate::RefreshInterval::TorrentPersistent;
                self.peer_algorithm = crate::algorithm::Algorithm::Regex;
                self.peer_pattern = String::from(r"b'-qB4130-[A-Za-z0-9_~\\\\(\\\\)\\\\!\\\\.\\\\*-]{12}'");
                self.uppercase_encoded_hex = false;
                self.num_want = 200; self.num_want_on_stop = 0;
                self.query = String::from("info_hash={infohash}&peer_id={peerid}&port={port}&uploaded={uploaded}&downloaded={downloaded}&left={left}&corrupt=0&key={key}&event={event}&numwant={numwant}&compact=1&no_peer_id=1&supportcrypto=1&redundant=0");
                self.encoding_exclusion_pattern = String::from(r"b'[A-Za-z0-9_~\\\\(\\\\)\\\\!\\\\.\\\\*-]'");
                self.peer_url_encode = false;
                self.user_agent = String::from("qBittorrent/4.1.3");
                self.connection = Some(String::from("close"));
                self.accept_language = String::from("");
            },
            ClientVersion::Qbittorrent_4_1_4 => {
                self.name = String::from("qbittorrent-4.1.4");
                self.key_algorithm = crate::algorithm::Algorithm::HashNoLeadingZero;
                self.key_uppercase = Some(true);
                self.key_refresh_on = crate::RefreshInterval::TorrentPersistent;
                self.peer_algorithm = crate::algorithm::Algorithm::Regex;
                self.peer_pattern = String::from(r"b'-qB4140-[A-Za-z0-9_~\\\\(\\\\)\\\\!\\\\.\\\\*-]{12}'");
                self.uppercase_encoded_hex = false;
                self.num_want = 200; self.num_want_on_stop = 0;
                self.query = String::from("info_hash={infohash}&peer_id={peerid}&port={port}&uploaded={uploaded}&downloaded={downloaded}&left={left}&corrupt=0&key={key}&event={event}&numwant={numwant}&compact=1&no_peer_id=1&supportcrypto=1&redundant=0");
                self.encoding_exclusion_pattern = String::from(r"b'[A-Za-z0-9_~\\\\(\\\\)\\\\!\\\\.\\\\*-]'");
                self.peer_url_encode = false;
                self.user_agent = String::from("qBittorrent/4.1.4");
                self.connection = Some(String::from("close"));
                self.accept_language = String::from("");
            },
            ClientVersion::Qbittorrent_4_1_5 => {
                self.name = String::from("qbittorrent-4.1.5");
                self.key_algorithm = crate::algorithm::Algorithm::HashNoLeadingZero;
                self.key_uppercase = Some(true);
                self.key_refresh_on = crate::RefreshInterval::TorrentPersistent;
                self.peer_algorithm = crate::algorithm::Algorithm::Regex;
                self.peer_pattern = String::from(r"b'-qB4150-[A-Za-z0-9_~\\\\(\\\\)\\\\!\\\\.\\\\*-]{12}'");
                self.uppercase_encoded_hex = false;
                self.num_want = 200; self.num_want_on_stop = 0;
                self.query = String::from("info_hash={infohash}&peer_id={peerid}&port={port}&uploaded={uploaded}&downloaded={downloaded}&left={left}&corrupt=0&key={key}&event={event}&numwant={numwant}&compact=1&no_peer_id=1&supportcrypto=1&redundant=0");
                self.encoding_exclusion_pattern = String::from(r"b'[A-Za-z0-9_~\\\\(\\\\)\\\\!\\\\.\\\\*-]'");
                self.peer_url_encode = false;
                self.user_agent = String::from("qBittorrent/4.1.5");
                self.connection = Some(String::from("close"));
                self.accept_language = String::from("");
            },
            ClientVersion::Qbittorrent_4_1_6 => {
                self.name = String::from("qbittorrent-4.1.6");
                self.key_algorithm = crate::algorithm::Algorithm::HashNoLeadingZero;
                self.key_uppercase = Some(true);
                self.key_refresh_on = crate::RefreshInterval::TorrentPersistent;
                self.peer_algorithm = crate::algorithm::Algorithm::Regex;
                self.peer_pattern = String::from(r"b'-qB4160-[A-Za-z0-9_~\\\\(\\\\)\\\\!\\\\.\\\\*-]{12}'");
                self.uppercase_encoded_hex = false;
                self.num_want = 200; self.num_want_on_stop = 0;
                self.query = String::from("info_hash={infohash}&peer_id={peerid}&port={port}&uploaded={uploaded}&downloaded={downloaded}&left={left}&corrupt=0&key={key}&event={event}&numwant={numwant}&compact=1&no_peer_id=1&supportcrypto=1&redundant=0");
                self.encoding_exclusion_pattern = String::from(r"b'[A-Za-z0-9_~\\\\(\\\\)\\\\!\\\\.\\\\*-]'");
                self.peer_url_encode = false;
                self.user_agent = String::from("qBittorrent/4.1.6");
                self.connection = Some(String::from("close"));
                self.accept_language = String::from("");
            },
            ClientVersion::Qbittorrent_4_1_7 => {
                self.name = String::from("qbittorrent-4.1.7");
                self.key_algorithm = crate::algorithm::Algorithm::HashNoLeadingZero;
                self.key_uppercase = Some(true);
                self.key_refresh_on = crate::RefreshInterval::TorrentPersistent;
                self.peer_algorithm = crate::algorithm::Algorithm::Regex;
                self.peer_pattern = String::from(r"b'-qB4170-[A-Za-z0-9_~\\\\(\\\\)\\\\!\\\\.\\\\*-]{12}'");
                self.uppercase_encoded_hex = false;
                self.num_want = 200; self.num_want_on_stop = 0;
                self.query = String::from("info_hash={infohash}&peer_id={peerid}&port={port}&uploaded={uploaded}&downloaded={downloaded}&left={left}&corrupt=0&key={key}&event={event}&numwant={numwant}&compact=1&no_peer_id=1&supportcrypto=1&redundant=0");
                self.encoding_exclusion_pattern = String::from(r"b'[A-Za-z0-9_~\\\\(\\\\)\\\\!\\\\.\\\\*-]'");
                self.peer_url_encode = false;
                self.user_agent = String::from("qBittorrent/4.1.7");
                self.connection = Some(String::from("close"));
                self.accept_language = String::from("");
            },
            ClientVersion::Qbittorrent_4_1_8 => {
                self.name = String::from("qbittorrent-4.1.8");
                self.key_algorithm = crate::algorithm::Algorithm::HashNoLeadingZero;
                self.key_uppercase = Some(true);
                self.key_refresh_on = crate::RefreshInterval::TorrentPersistent;
                self.peer_algorithm = crate::algorithm::Algorithm::Regex;
                self.peer_pattern = String::from(r"b'-qB4180-[A-Za-z0-9_~\\\\(\\\\)\\\\!\\\\.\\\\*-]{12}'");
                self.uppercase_encoded_hex = false;
                self.num_want = 200; self.num_want_on_stop = 0;
                self.query = String::from("info_hash={infohash}&peer_id={peerid}&port={port}&uploaded={uploaded}&downloaded={downloaded}&left={left}&corrupt=0&key={key}&event={event}&numwant={numwant}&compact=1&no_peer_id=1&supportcrypto=1&redundant=0");
                self.encoding_exclusion_pattern = String::from(r"b'[A-Za-z0-9_~\\\\(\\\\)\\\\!\\\\.\\\\*-]'");
                self.peer_url_encode = false;
                self.user_agent = String::from("qBittorrent/4.1.8");
                self.connection = Some(String::from("close"));
                self.accept_language = String::from("");
            },
            ClientVersion::Qbittorrent_4_1_9 => {
                self.name = String::from("qbittorrent-4.1.9");
                self.key_algorithm = crate::algorithm::Algorithm::HashNoLeadingZero;
                self.key_uppercase = Some(true);
                self.key_refresh_on = crate::RefreshInterval::TorrentPersistent;
                self.peer_algorithm = crate::algorithm::Algorithm::Regex;
                self.peer_pattern = String::from(r"b'-qB4190-[A-Za-z0-9_~\\\\(\\\\)\\\\!\\\\.\\\\*-]{12}'");
                self.uppercase_encoded_hex = false;
                self.num_want = 200; self.num_want_on_stop = 0;
                self.query = String::from("info_hash={infohash}&peer_id={peerid}&port={port}&uploaded={uploaded}&downloaded={downloaded}&left={left}&corrupt=0&key={key}&event={event}&numwant={numwant}&compact=1&no_peer_id=1&supportcrypto=1&redundant=0");
                self.encoding_exclusion_pattern = String::from(r"b'[A-Za-z0-9_~\\\\(\\\\)\\\\!\\\\.\\\\*-]'");
                self.peer_url_encode = false;
                self.user_agent = String::from("qBittorrent/4.1.9");
                self.connection = Some(String::from("close"));
                self.accept_language = String::from("");
            },
            ClientVersion::Qbittorrent_4_2_0 => {
                self.name = String::from("qbittorrent-4.2.0");
                self.key_algorithm = crate::algorithm::Algorithm::HashNoLeadingZero;
                self.key_uppercase = Some(true);
                self.key_refresh_on = crate::RefreshInterval::TorrentPersistent;
                self.peer_algorithm = crate::algorithm::Algorithm::Regex;
                self.peer_pattern = String::from(r"b'-qB4200-[A-Za-z0-9_~\\\\(\\\\)\\\\!\\\\.\\\\*-]{12}'");
                self.uppercase_encoded_hex = false;
                self.num_want = 200; self.num_want_on_stop = 0;
                self.query = String::from("info_hash={infohash}&peer_id={peerid}&port={port}&uploaded={uploaded}&downloaded={downloaded}&left={left}&corrupt=0&key={key}&event={event}&numwant={numwant}&compact=1&no_peer_id=1&supportcrypto=1&redundant=0");
                self.encoding_exclusion_pattern = String::from(r"b'[A-Za-z0-9_~\\\\(\\\\)\\\\!\\\\.\\\\*-]'");
                self.peer_url_encode = false;
                self.user_agent = String::from("qBittorrent/4.2.0");
                self.connection = Some(String::from("close"));
                self.accept_language = String::from("");
            },
            ClientVersion::Qbittorrent_4_2_1 => {
                self.name = String::from("qbittorrent-4.2.1");
                self.key_algorithm = crate::algorithm::Algorithm::HashNoLeadingZero;
                self.key_uppercase = Some(true);
                self.key_refresh_on = crate::RefreshInterval::TorrentPersistent;
                self.peer_algorithm = crate::algorithm::Algorithm::Regex;
                self.peer_pattern = String::from(r"b'-qB4210-[A-Za-z0-9_~\\\\(\\\\)\\\\!\\\\.\\\\*-]{12}'");
                self.uppercase_encoded_hex = false;
                self.num_want = 200; self.num_want_on_stop = 0;
                self.query = String::from("info_hash={infohash}&peer_id={peerid}&port={port}&uploaded={uploaded}&downloaded={downloaded}&left={left}&corrupt=0&key={key}&event={event}&numwant={numwant}&compact=1&no_peer_id=1&supportcrypto=1&redundant=0");
                self.encoding_exclusion_pattern = String::from(r"b'[A-Za-z0-9_~\\\\(\\\\)\\\\!\\\\.\\\\*-]'");
                self.peer_url_encode = false;
                self.user_agent = String::from("qBittorrent/4.2.1");
                self.connection = Some(String::from("close"));
                self.accept_language = String::from("");
            },
            ClientVersion::Qbittorrent_4_2_2 => {
                self.name = String::from("qbittorrent-4.2.2");
                self.key_algorithm = crate::algorithm::Algorithm::HashNoLeadingZero;
                self.key_uppercase = Some(true);
                self.key_refresh_on = crate::RefreshInterval::TorrentPersistent;
                self.peer_algorithm = crate::algorithm::Algorithm::Regex;
                self.peer_pattern = String::from(r"b'-qB4220-[A-Za-z0-9_~\\\\(\\\\)\\\\!\\\\.\\\\*-]{12}'");
                self.uppercase_encoded_hex = false;
                self.num_want = 200; self.num_want_on_stop = 0;
                self.query = String::from("info_hash={infohash}&peer_id={peerid}&port={port}&uploaded={uploaded}&downloaded={downloaded}&left={left}&corrupt=0&key={key}&event={event}&numwant={numwant}&compact=1&no_peer_id=1&supportcrypto=1&redundant=0");
                self.encoding_exclusion_pattern = String::from(r"b'[A-Za-z0-9_~\\\\(\\\\)\\\\!\\\\.\\\\*-]'");
                self.peer_url_encode = false;
                self.user_agent = String::from("qBittorrent/4.2.2");
                self.connection = Some(String::from("close"));
                self.accept_language = String::from("");
            },
            ClientVersion::Qbittorrent_4_2_3 => {
                self.name = String::from("qbittorrent-4.2.3");
                self.key_algorithm = crate::algorithm::Algorithm::HashNoLeadingZero;
                self.key_uppercase = Some(true);
                self.key_refresh_on = crate::RefreshInterval::TorrentPersistent;
                self.peer_algorithm = crate::algorithm::Algorithm::Regex;
                self.peer_pattern = String::from(r"b'-qB4230-[A-Za-z0-9_~\\\\(\\\\)\\\\!\\\\.\\\\*-]{12}'");
                self.uppercase_encoded_hex = false;
                self.num_want = 200; self.num_want_on_stop = 0;
                self.query = String::from("info_hash={infohash}&peer_id={peerid}&port={port}&uploaded={uploaded}&downloaded={downloaded}&left={left}&corrupt=0&key={key}&event={event}&numwant={numwant}&compact=1&no_peer_id=1&supportcrypto=1&redundant=0");
                self.encoding_exclusion_pattern = String::from(r"b'[A-Za-z0-9_~\\\\(\\\\)\\\\!\\\\.\\\\*-]'");
                self.peer_url_encode = false;
                self.user_agent = String::from("qBittorrent/4.2.3");
                self.connection = Some(String::from("close"));
                self.accept_language = String::from("");
            },
            ClientVersion::Qbittorrent_4_2_4 => {
                self.name = String::from("qbittorrent-4.2.4");
                self.key_algorithm = crate::algorithm::Algorithm::HashNoLeadingZero;
                self.key_uppercase = Some(true);
                self.key_refresh_on = crate::RefreshInterval::TorrentPersistent;
                self.peer_algorithm = crate::algorithm::Algorithm::Regex;
                self.peer_pattern = String::from(r"b'-qB4240-[A-Za-z0-9_~\\\\(\\\\)\\\\!\\\\.\\\\*-]{12}'");
                self.uppercase_encoded_hex = false;
                self.num_want = 200; self.num_want_on_stop = 0;
                self.query = String::from("info_hash={infohash}&peer_id={peerid}&port={port}&uploaded={uploaded}&downloaded={downloaded}&left={left}&corrupt=0&key={key}&event={event}&numwant={numwant}&compact=1&no_peer_id=1&supportcrypto=1&redundant=0");
                self.encoding_exclusion_pattern = String::from(r"b'[A-Za-z0-9_~\\\\(\\\\)\\\\!\\\\.\\\\*-]'");
                self.peer_url_encode = false;
                self.user_agent = String::from("qBittorrent/4.2.4");
                self.connection = Some(String::from("close"));
                self.accept_language = String::from("");
            },
            ClientVersion::Qbittorrent_4_2_5 => {
                self.name = String::from("qbittorrent-4.2.5");
                self.key_algorithm = crate::algorithm::Algorithm::HashNoLeadingZero;
                self.key_uppercase = Some(true);
                self.key_refresh_on = crate::RefreshInterval::TorrentPersistent;
                self.peer_algorithm = crate::algorithm::Algorithm::Regex;
                self.peer_pattern = String::from(r"b'-qB4250-[A-Za-z0-9_~\\\\(\\\\)\\\\!\\\\.\\\\*-]{12}'");
                self.uppercase_encoded_hex = false;
                self.num_want = 200; self.num_want_on_stop = 0;
                self.query = String::from("info_hash={infohash}&peer_id={peerid}&port={port}&uploaded={uploaded}&downloaded={downloaded}&left={left}&corrupt=0&key={key}&event={event}&numwant={numwant}&compact=1&no_peer_id=1&supportcrypto=1&redundant=0");
                self.encoding_exclusion_pattern = String::from(r"b'[A-Za-z0-9_~\\\\(\\\\)\\\\!\\\\.\\\\*-]'");
                self.peer_url_encode = false;
                self.user_agent = String::from("qBittorrent/4.2.5");
                self.connection = Some(String::from("close"));
                self.accept_language = String::from("");
            },
            ClientVersion::Qbittorrent_4_3_0_1 => {
                self.name = String::from("qbittorrent-4.3.0.1");
                self.key_algorithm = crate::algorithm::Algorithm::HashNoLeadingZero;
                self.key_uppercase = Some(true);
                self.key_refresh_on = crate::RefreshInterval::TorrentPersistent;
                self.peer_algorithm = crate::algorithm::Algorithm::Regex;
                self.peer_pattern = String::from(r"b'-qB4301-[A-Za-z0-9_~\\\\(\\\\)\\\\!\\\\.\\\\*-]{12}'");
                self.uppercase_encoded_hex = false;
                self.num_want = 200; self.num_want_on_stop = 0;
                self.query = String::from("info_hash={infohash}&peer_id={peerid}&port={port}&uploaded={uploaded}&downloaded={downloaded}&left={left}&corrupt=0&key={key}&event={event}&numwant={numwant}&compact=1&no_peer_id=1&supportcrypto=1&redundant=0");
                self.encoding_exclusion_pattern = String::from(r"b'[A-Za-z0-9_~\\\\(\\\\)\\\\!\\\\.\\\\*-]'");
                self.peer_url_encode = false;
                self.user_agent = String::from("qBittorrent/4.3.0.1");
                self.connection = Some(String::from("close"));
                self.accept_language = String::from("");
            },
            ClientVersion::Qbittorrent_4_3_0 => {
                self.name = String::from("qbittorrent-4.3.0");
                self.key_algorithm = crate::algorithm::Algorithm::HashNoLeadingZero;
                self.key_uppercase = Some(true);
                self.key_refresh_on = crate::RefreshInterval::TorrentPersistent;
                self.peer_algorithm = crate::algorithm::Algorithm::Regex;
                self.peer_pattern = String::from(r"b'-qB4300-[A-Za-z0-9_~\\\\(\\\\)\\\\!\\\\.\\\\*-]{12}'");
                self.uppercase_encoded_hex = false;
                self.num_want = 200; self.num_want_on_stop = 0;
                self.query = String::from("info_hash={infohash}&peer_id={peerid}&port={port}&uploaded={uploaded}&downloaded={downloaded}&left={left}&corrupt=0&key={key}&event={event}&numwant={numwant}&compact=1&no_peer_id=1&supportcrypto=1&redundant=0");
                self.encoding_exclusion_pattern = String::from(r"b'[A-Za-z0-9_~\\\\(\\\\)\\\\!\\\\.\\\\*-]'");
                self.peer_url_encode = false;
                self.user_agent = String::from("qBittorrent/4.3.0");
                self.connection = Some(String::from("close"));
                self.accept_language = String::from("");
            },
            ClientVersion::Qbittorrent_4_3_1 => {
                self.name = String::from("qbittorrent-4.3.1");
                self.key_algorithm = crate::algorithm::Algorithm::HashNoLeadingZero;
                self.key_uppercase = Some(true);
                self.key_refresh_on = crate::RefreshInterval::TorrentPersistent;
                self.peer_algorithm = crate::algorithm::Algorithm::Regex;
                self.peer_pattern = String::from(r"b'-qB4310-[A-Za-z0-9_~\\\\(\\\\)\\\\!\\\\.\\\\*-]{12}'");
                self.uppercase_encoded_hex = false;
                self.num_want = 200; self.num_want_on_stop = 0;
                self.query = String::from("info_hash={infohash}&peer_id={peerid}&port={port}&uploaded={uploaded}&downloaded={downloaded}&left={left}&corrupt=0&key={key}&event={event}&numwant={numwant}&compact=1&no_peer_id=1&supportcrypto=1&redundant=0");
                self.encoding_exclusion_pattern = String::from(r"b'[A-Za-z0-9_~\\\\(\\\\)\\\\!\\\\.\\\\*-]'");
                self.peer_url_encode = false;
                self.user_agent = String::from("qBittorrent/4.3.1");
                self.connection = Some(String::from("close"));
                self.accept_language = String::from("");
            },
            ClientVersion::Qbittorrent_4_3_2 => {
                self.name = String::from("qbittorrent-4.3.2");
                self.key_algorithm = crate::algorithm::Algorithm::HashNoLeadingZero;
                self.key_uppercase = Some(true);
                self.key_refresh_on = crate::RefreshInterval::TorrentPersistent;
                self.peer_algorithm = crate::algorithm::Algorithm::Regex;
                self.peer_pattern = String::from(r"b'-qB4320-[A-Za-z0-9_~\\\\(\\\\)\\\\!\\\\.\\\\*-]{12}'");
                self.uppercase_encoded_hex = false;
                self.num_want = 200; self.num_want_on_stop = 0;
                self.query = String::from("info_hash={infohash}&peer_id={peerid}&port={port}&uploaded={uploaded}&downloaded={downloaded}&left={left}&corrupt=0&key={key}&event={event}&numwant={numwant}&compact=1&no_peer_id=1&supportcrypto=1&redundant=0");
                self.encoding_exclusion_pattern = String::from(r"b'[A-Za-z0-9_~\\\\(\\\\)\\\\!\\\\.\\\\*-]'");
                self.peer_url_encode = false;
                self.user_agent = String::from("qBittorrent/4.3.2");
                self.connection = Some(String::from("close"));
                self.accept_language = String::from("");
            },
            ClientVersion::Qbittorrent_4_3_3 => {
                self.name = String::from("qbittorrent-4.3.3");
                self.key_algorithm = crate::algorithm::Algorithm::HashNoLeadingZero;
                self.key_uppercase = Some(true);
                self.key_refresh_on = crate::RefreshInterval::TorrentPersistent;
                self.peer_algorithm = crate::algorithm::Algorithm::Regex;
                self.peer_pattern = String::from(r"b'-qB4330-[A-Za-z0-9_~\\\\(\\\\)\\\\!\\\\.\\\\*-]{12}'");
                self.uppercase_encoded_hex = false;
                self.num_want = 200; self.num_want_on_stop = 0;
                self.query = String::from("info_hash={infohash}&peer_id={peerid}&port={port}&uploaded={uploaded}&downloaded={downloaded}&left={left}&corrupt=0&key={key}&event={event}&numwant={numwant}&compact=1&no_peer_id=1&supportcrypto=1&redundant=0");
                self.encoding_exclusion_pattern = String::from(r"b'[A-Za-z0-9_~\\\\(\\\\)\\\\!\\\\.\\\\*-]'");
                self.peer_url_encode = false;
                self.user_agent = String::from("qBittorrent/4.3.3");
                self.connection = Some(String::from("close"));
                self.accept_language = String::from("");
            },
            ClientVersion::Qbittorrent_4_3_4_1 => {
                self.name = String::from("qbittorrent-4.3.4.1");
                self.key_algorithm = crate::algorithm::Algorithm::HashNoLeadingZero;
                self.key_uppercase = Some(true);
                self.key_refresh_on = crate::RefreshInterval::TorrentPersistent;
                self.peer_algorithm = crate::algorithm::Algorithm::Regex;
                self.peer_pattern = String::from(r"b'-qB4341-[A-Za-z0-9_~\\\\(\\\\)\\\\!\\\\.\\\\*-]{12}'");
                self.uppercase_encoded_hex = false;
                self.num_want = 200; self.num_want_on_stop = 0;
                self.query = String::from("info_hash={infohash}&peer_id={peerid}&port={port}&uploaded={uploaded}&downloaded={downloaded}&left={left}&corrupt=0&key={key}&event={event}&numwant={numwant}&compact=1&no_peer_id=1&supportcrypto=1&redundant=0");
                self.encoding_exclusion_pattern = String::from(r"b'[A-Za-z0-9_~\\\\(\\\\)\\\\!\\\\.\\\\*-]'");
                self.peer_url_encode = false;
                self.user_agent = String::from("qBittorrent/4.3.4.1");
                self.connection = Some(String::from("close"));
                self.accept_language = String::from("");
            },
            ClientVersion::Qbittorrent_4_3_5 => {
                self.name = String::from("qbittorrent-4.3.5");
                self.key_algorithm = crate::algorithm::Algorithm::HashNoLeadingZero;
                self.key_uppercase = Some(true);
                self.key_refresh_on = crate::RefreshInterval::TorrentPersistent;
                self.peer_algorithm = crate::algorithm::Algorithm::Regex;
                self.peer_pattern = String::from(r"b'-qB4350-[A-Za-z0-9_~\\\\(\\\\)\\\\!\\\\.\\\\*-]{12}'");
                self.uppercase_encoded_hex = false;
                self.num_want = 200; self.num_want_on_stop = 0;
                self.query = String::from("info_hash={infohash}&peer_id={peerid}&port={port}&uploaded={uploaded}&downloaded={downloaded}&left={left}&corrupt=0&key={key}&event={event}&numwant={numwant}&compact=1&no_peer_id=1&supportcrypto=1&redundant=0");
                self.encoding_exclusion_pattern = String::from(r"b'[A-Za-z0-9_~\\\\(\\\\)\\\\!\\\\.\\\\*-]'");
                self.peer_url_encode = false;
                self.user_agent = String::from("qBittorrent/4.3.5");
                self.connection = Some(String::from("close"));
                self.accept_language = String::from("");
            },
            ClientVersion::Qbittorrent_4_3_6 => {
                self.name = String::from("qbittorrent-4.3.6");
                self.key_algorithm = crate::algorithm::Algorithm::HashNoLeadingZero;
                self.key_uppercase = Some(true);
                self.key_refresh_on = crate::RefreshInterval::TorrentPersistent;
                self.peer_algorithm = crate::algorithm::Algorithm::Regex;
                self.peer_pattern = String::from(r"b'-qB4360-[A-Za-z0-9_~\\\\(\\\\)\\\\!\\\\.\\\\*-]{12}'");
                self.uppercase_encoded_hex = false;
                self.num_want = 200; self.num_want_on_stop = 0;
                self.query = String::from("info_hash={infohash}&peer_id={peerid}&port={port}&uploaded={uploaded}&downloaded={downloaded}&left={left}&corrupt=0&key={key}&event={event}&numwant={numwant}&compact=1&no_peer_id=1&supportcrypto=1&redundant=0");
                self.encoding_exclusion_pattern = String::from(r"b'[A-Za-z0-9_~\\\\(\\\\)\\\\!\\\\.\\\\*-]'");
                self.peer_url_encode = false;
                self.user_agent = String::from("qBittorrent/4.3.6");
                self.connection = Some(String::from("close"));
                self.accept_language = String::from("");
            },
            ClientVersion::Qbittorrent_4_3_8 => {
                self.name = String::from("qbittorrent-4.3.8");
                self.key_algorithm = crate::algorithm::Algorithm::HashNoLeadingZero;
                self.key_uppercase = Some(true);
                self.key_refresh_on = crate::RefreshInterval::TorrentPersistent;
                self.peer_algorithm = crate::algorithm::Algorithm::Regex;
                self.peer_pattern = String::from(r"b'-qB4380-[A-Za-z0-9_~\\\\(\\\\)\\\\!\\\\.\\\\*-]{12}'");
                self.uppercase_encoded_hex = false;
                self.num_want = 200; self.num_want_on_stop = 0;
                self.query = String::from("info_hash={infohash}&peer_id={peerid}&port={port}&uploaded={uploaded}&downloaded={downloaded}&left={left}&corrupt=0&key={key}&event={event}&numwant={numwant}&compact=1&no_peer_id=1&supportcrypto=1&redundant=0");
                self.encoding_exclusion_pattern = String::from(r"b'[A-Za-z0-9_~\\\\(\\\\)\\\\!\\\\.\\\\*-]'");
                self.peer_url_encode = false;
                self.user_agent = String::from("qBittorrent/4.3.8");
                self.connection = Some(String::from("close"));
                self.accept_language = String::from("");
            },
            ClientVersion::Qbittorrent_4_3_9 => {
                self.name = String::from("qbittorrent-4.3.9");
                self.key_algorithm = crate::algorithm::Algorithm::HashNoLeadingZero;
                self.key_uppercase = Some(true);
                self.key_refresh_on = crate::RefreshInterval::TorrentPersistent;
                self.peer_algorithm = crate::algorithm::Algorithm::Regex;
                self.peer_pattern = String::from(r"b'-qB4390-[A-Za-z0-9_~\\\\(\\\\)\\\\!\\\\.\\\\*-]{12}'");
                self.uppercase_encoded_hex = false;
                self.num_want = 200; self.num_want_on_stop = 0;
                self.query = String::from("info_hash={infohash}&peer_id={peerid}&port={port}&uploaded={uploaded}&downloaded={downloaded}&left={left}&corrupt=0&key={key}&event={event}&numwant={numwant}&compact=1&no_peer_id=1&supportcrypto=1&redundant=0");
                self.encoding_exclusion_pattern = String::from(r"b'[A-Za-z0-9_~\\\\(\\\\)\\\\!\\\\.\\\\*-]'");
                self.peer_url_encode = false;
                self.user_agent = String::from("qBittorrent/4.3.9");
                self.connection = Some(String::from("close"));
                self.accept_language = String::from("");
            },
            ClientVersion::Qbittorrent_4_4_2 => {
                self.name = String::from("qbittorrent-4.4.2");
                self.key_algorithm = crate::algorithm::Algorithm::HashNoLeadingZero;
                self.key_uppercase = Some(true);
                self.key_refresh_on = crate::RefreshInterval::TorrentPersistent;
                self.peer_algorithm = crate::algorithm::Algorithm::Regex;
                self.peer_pattern = String::from(r"b'-qB4420-[A-Za-z0-9_~\\\\(\\\\)\\\\!\\\\.\\\\*-]{12}'");
                self.uppercase_encoded_hex = false;
                self.num_want = 200; self.num_want_on_stop = 0;
                self.query = String::from("info_hash={infohash}&peer_id={peerid}&port={port}&uploaded={uploaded}&downloaded={downloaded}&left={left}&corrupt=0&key={key}&event={event}&numwant={numwant}&compact=1&no_peer_id=1&supportcrypto=1&redundant=0");
                self.encoding_exclusion_pattern = String::from(r"b'[A-Za-z0-9_~\\\\(\\\\)\\\\!\\\\.\\\\*-]'");
                self.peer_url_encode = false;
                self.user_agent = String::from("qBittorrent/4.4.2");
                self.connection = Some(String::from("close"));
                self.accept_language = String::from("");
            },
            ClientVersion::Qbittorrent_4_4_3_1 => {
                self.name = String::from("qbittorrent-4.4.3.1");
                self.key_algorithm = crate::algorithm::Algorithm::HashNoLeadingZero;
                self.key_uppercase = Some(true);
                self.key_refresh_on = crate::RefreshInterval::TorrentPersistent;
                self.peer_algorithm = crate::algorithm::Algorithm::Regex;
                self.peer_pattern = String::from(r"b'-qB4431-[A-Za-z0-9_~\\\\(\\\\)\\\\!\\\\.\\\\*-]{12}'");
                self.uppercase_encoded_hex = false;
                self.num_want = 200; self.num_want_on_stop = 0;
                self.query = String::from("info_hash={infohash}&peer_id={peerid}&port={port}&uploaded={uploaded}&downloaded={downloaded}&left={left}&corrupt=0&key={key}&event={event}&numwant={numwant}&compact=1&no_peer_id=1&supportcrypto=1&redundant=0");
                self.encoding_exclusion_pattern = String::from(r"b'[A-Za-z0-9_~\\\\(\\\\)\\\\!\\\\.\\\\*-]'");
                self.peer_url_encode = false;
                self.user_agent = String::from("qBittorrent/4.4.3.1");
                self.connection = Some(String::from("close"));
                self.accept_language = String::from("");
            },
            ClientVersion::Rtorrent_0_9_6_0_13_6 => {
                self.name = String::from("rtorrent-0.9.6_0.13.6");
                self.key_algorithm = crate::algorithm::Algorithm::Hash;
                self.key_uppercase = Some(false);
                self.key_refresh_on = crate::RefreshInterval::TorrentPersistent;
                self.peer_algorithm = crate::algorithm::Algorithm::Regex;
                self.peer_pattern = String::from(r"b'-lt0D60-[\x01-\xc3\xbf]{12}'");
                self.peer_refresh_on = crate::RefreshInterval::TorrentPersistent;
                self.uppercase_encoded_hex = true;
                self.num_want = 50; self.num_want_on_stop = 0;
                self.query = String::from("info_hash={infohash}&peer_id={peerid}&key={key}&compact=1&port={port}&uploaded={uploaded}&downloaded={downloaded}&left={left}&event={event}");
                self.encoding_exclusion_pattern = String::from(r"b'[A-Za-z0-9-]'");
                self.peer_url_encode = true;
                self.user_agent = String::from("rtorrent/0.9.6/0.13.6");
                self.accept_encoding = String::from("deflate, gzip");
                self.accept = String::from("*/*");
                self.accept_language = String::from("");
            },
            ClientVersion::Transmission_2_82_14160 => {
                self.name = String::from("transmission-2.82_14160");
                self.key_algorithm = crate::algorithm::Algorithm::DigitRangeTransformedToHexWithoutLeadingZeroes;
                self.key_uppercase = Some(false);
                self.key_refresh_on = crate::RefreshInterval::Never;
                self.peer_algorithm = crate::algorithm::Algorithm::RandomPoolWithChecksum;
                self.peer_pattern = String::from(r"b'0123456789abcdefghijklmnopqrstuvwxyz'");
                self.peer_prefix = String::from("-TR2820-");
                self.peer_refresh_on = crate::RefreshInterval::TorrentVolatile;
                self.uppercase_encoded_hex = false;
                self.num_want = 80; self.num_want_on_stop = 0;
                self.query = String::from("info_hash={infohash}&peer_id={peerid}&port={port}&uploaded={uploaded}&downloaded={downloaded}&left={left}&numwant={numwant}&key={key}&compact=1&supportcrypto=1&event={event}&ipv6={ipv6}");
                self.encoding_exclusion_pattern = String::from(r"b'[A-Za-z0-9-]'");
                self.peer_url_encode = false;
                self.user_agent = String::from("Transmission/2.82");
                self.accept_encoding = String::from("gzip;q=1.0, deflate, identity");
                self.accept = String::from("*/*");
                self.accept_language = String::from("");
            },
            ClientVersion::Transmission_2_92_14714 => {
                self.name = String::from("transmission-2.92_14714");
                self.key_algorithm = crate::algorithm::Algorithm::DigitRangeTransformedToHexWithoutLeadingZeroes;
                self.key_uppercase = Some(false);
                self.key_refresh_on = crate::RefreshInterval::Never;
                self.peer_algorithm = crate::algorithm::Algorithm::RandomPoolWithChecksum;
                self.peer_pattern = String::from(r"b'0123456789abcdefghijklmnopqrstuvwxyz'");
                self.peer_prefix = String::from("-TR284Z-");
                self.peer_refresh_on = crate::RefreshInterval::TorrentVolatile;
                self.uppercase_encoded_hex = false;
                self.num_want = 80; self.num_want_on_stop = 0;
                self.query = String::from("info_hash={infohash}&peer_id={peerid}&port={port}&uploaded={uploaded}&downloaded={downloaded}&left={left}&numwant={numwant}&key={key}&compact=1&supportcrypto=1&event={event}&ipv6={ipv6}");
                self.encoding_exclusion_pattern = String::from(r"b'[A-Za-z0-9-]'");
                self.peer_url_encode = false;
                self.user_agent = String::from("Transmission/2.84+");
                self.accept_encoding = String::from("gzip;q=1.0, deflate, identity");
                self.accept = String::from("*/*");
                self.accept_language = String::from("");
            },
            ClientVersion::Transmission_2_93 => {
                self.name = String::from("transmission-2.93");
                self.key_algorithm = crate::algorithm::Algorithm::DigitRangeTransformedToHexWithoutLeadingZeroes;
                self.key_uppercase = Some(false);
                self.key_refresh_on = crate::RefreshInterval::Never;
                self.peer_algorithm = crate::algorithm::Algorithm::RandomPoolWithChecksum;
                self.peer_pattern = String::from(r"b'0123456789abcdefghijklmnopqrstuvwxyz'");
                self.peer_prefix = String::from("-TR2930-");
                self.peer_refresh_on = crate::RefreshInterval::TorrentVolatile;
                self.uppercase_encoded_hex = false;
                self.num_want = 80; self.num_want_on_stop = 0;
                self.query = String::from("info_hash={infohash}&peer_id={peerid}&port={port}&uploaded={uploaded}&downloaded={downloaded}&left={left}&numwant={numwant}&key={key}&compact=1&supportcrypto=1&event={event}&ipv6={ipv6}");
                self.encoding_exclusion_pattern = String::from(r"b'[A-Za-z0-9-]'");
                self.peer_url_encode = false;
                self.user_agent = String::from("Transmission/2.93");
                self.accept_encoding = String::from("gzip;q=1.0, deflate, identity");
                self.accept = String::from("*/*");
                self.accept_language = String::from("");
            },
            ClientVersion::Transmission_2_94 => {
                self.name = String::from("transmission-2.94");
                self.key_algorithm = crate::algorithm::Algorithm::DigitRangeTransformedToHexWithoutLeadingZeroes;
                self.key_uppercase = Some(false);
                self.key_refresh_on = crate::RefreshInterval::Never;
                self.peer_algorithm = crate::algorithm::Algorithm::RandomPoolWithChecksum;
                self.peer_pattern = String::from(r"b'0123456789abcdefghijklmnopqrstuvwxyz'");
                self.peer_prefix = String::from("-TR2940-");
                self.peer_refresh_on = crate::RefreshInterval::TorrentVolatile;
                self.uppercase_encoded_hex = false;
                self.num_want = 80; self.num_want_on_stop = 0;
                self.query = String::from("info_hash={infohash}&peer_id={peerid}&port={port}&uploaded={uploaded}&downloaded={downloaded}&left={left}&numwant={numwant}&key={key}&compact=1&supportcrypto=1&event={event}&ipv6={ipv6}");
                self.encoding_exclusion_pattern = String::from(r"b'[A-Za-z0-9-]'");
                self.peer_url_encode = false;
                self.user_agent = String::from("Transmission/2.94");
                self.accept_encoding = String::from("gzip;q=1.0, deflate, identity");
                self.accept = String::from("*/*");
                self.accept_language = String::from("");
            },
            ClientVersion::Transmission_3_00 => {
                self.name = String::from("transmission-3.00");
                self.key_algorithm = crate::algorithm::Algorithm::DigitRangeTransformedToHexWithoutLeadingZeroes;
                self.key_uppercase = Some(false);
                self.key_refresh_on = crate::RefreshInterval::Never;
                self.peer_algorithm = crate::algorithm::Algorithm::RandomPoolWithChecksum;
                self.peer_pattern = String::from(r"b'0123456789abcdefghijklmnopqrstuvwxyz'");
                self.peer_prefix = String::from("-TR3000-");
                self.peer_refresh_on = crate::RefreshInterval::TorrentVolatile;
                self.uppercase_encoded_hex = false;
                self.num_want = 80; self.num_want_on_stop = 0;
                self.query = String::from("info_hash={infohash}&peer_id={peerid}&port={port}&uploaded={uploaded}&downloaded={downloaded}&left={left}&numwant={numwant}&key={key}&compact=1&supportcrypto=1&event={event}&ipv6={ipv6}");
                self.encoding_exclusion_pattern = String::from(r"b'[A-Za-z0-9-]'");
                self.peer_url_encode = false;
                self.user_agent = String::from("Transmission/3.00");
                self.accept_encoding = String::from("deflate, gzip");
                self.accept = String::from("*/*");
                self.accept_language = String::from("");
            },
            ClientVersion::Utorrent_3_2_2_28500 => {
                self.name = String::from("utorrent-3.2.2_28500");
                self.key_algorithm = crate::algorithm::Algorithm::Hash;
                self.key_uppercase = Some(true);
                self.key_refresh_on = crate::RefreshInterval::TimedOrAfterStartedAnnounce;
                self.key_refresh_every = Some(10);
                self.peer_algorithm = crate::algorithm::Algorithm::Regex;
                self.peer_pattern = String::from(r"b'-UT3220-To[\x01-\xc3\xbf]{10}'");
                self.uppercase_encoded_hex = false;
                self.num_want = 200; self.num_want_on_stop = 200;
                self.query = String::from("info_hash={infohash}&peer_id={peerid}&port={port}&uploaded={uploaded}&downloaded={downloaded}&left={left}&corrupt=0&key={key}&event={event}&numwant={numwant}&compact=1&no_peer_id=1&ipv6={ipv6}");
                self.encoding_exclusion_pattern = String::from(r"b'[A-Za-z0-9-]'");
                self.peer_url_encode = true;
                self.user_agent = String::from("uTorrent/3220(28500)");
                self.connection = Some(String::from("Close"));
                self.accept_language = String::from("{locale}");
            },
            ClientVersion::Utorrent_3_5_0_43916 => {
                self.name = String::from("utorrent-3.5.0_43916");
                self.key_algorithm = crate::algorithm::Algorithm::Hash;
                self.key_uppercase = Some(true);
                self.key_refresh_on = crate::RefreshInterval::TimedOrAfterStartedAnnounce;
                self.key_refresh_every = Some(10);
                self.peer_algorithm = crate::algorithm::Algorithm::Regex;
                self.peer_pattern = String::from(r"b'-UT3500-(\xc2\x8c\xc2\xab)[\x01-\xc3\xbf]{10}'");
                self.uppercase_encoded_hex = false;
                self.num_want = 200; self.num_want_on_stop = 0;
                self.query = String::from("info_hash={infohash}&peer_id={peerid}&port={port}&uploaded={uploaded}&downloaded={downloaded}&left={left}&corrupt=0&key={key}&event={event}&numwant={numwant}&compact=1&no_peer_id=1");
                self.encoding_exclusion_pattern = String::from(r"b'[A-Za-z0-9-]'");
                self.peer_url_encode = true;
                self.user_agent = String::from("uTorrent/350(111258508)(43916)");
                self.connection = Some(String::from("Close"));
                self.accept_language = String::from("");
            },
            ClientVersion::Utorrent_3_5_0_44090 => {
                self.name = String::from("utorrent-3.5.0_44090");
                self.key_algorithm = crate::algorithm::Algorithm::Hash;
                self.key_uppercase = Some(true);
                self.key_refresh_on = crate::RefreshInterval::TimedOrAfterStartedAnnounce;
                self.key_refresh_every = Some(10);
                self.peer_algorithm = crate::algorithm::Algorithm::Regex;
                self.peer_pattern = String::from(r"b'-UT3500-(:\xc2\xac)[\x01-\xc3\xbf]{10}'");
                self.uppercase_encoded_hex = false;
                self.num_want = 200; self.num_want_on_stop = 0;
                self.query = String::from("info_hash={infohash}&peer_id={peerid}&port={port}&uploaded={uploaded}&downloaded={downloaded}&left={left}&corrupt=0&key={key}&event={event}&numwant={numwant}&compact=1&no_peer_id=1");
                self.encoding_exclusion_pattern = String::from(r"b'[A-Za-z0-9-]'");
                self.peer_url_encode = true;
                self.user_agent = String::from("uTorrent/350(111258682)(44090)");
                self.connection = Some(String::from("Close"));
                self.accept_language = String::from("");
            },
            ClientVersion::Utorrent_3_5_0_44294 => {
                self.name = String::from("utorrent-3.5.0_44294");
                self.key_algorithm = crate::algorithm::Algorithm::Hash;
                self.key_uppercase = Some(true);
                self.key_refresh_on = crate::RefreshInterval::TimedOrAfterStartedAnnounce;
                self.key_refresh_every = Some(10);
                self.peer_algorithm = crate::algorithm::Algorithm::Regex;
                self.peer_pattern = String::from(r"b'-UT3500-(\x06\xc2\xad)[\x01-\xc3\xbf]{10}'");
                self.uppercase_encoded_hex = false;
                self.num_want = 200; self.num_want_on_stop = 0;
                self.query = String::from("info_hash={infohash}&peer_id={peerid}&port={port}&uploaded={uploaded}&downloaded={downloaded}&left={left}&corrupt=0&key={key}&event={event}&numwant={numwant}&compact=1&no_peer_id=1");
                self.encoding_exclusion_pattern = String::from(r"b'[A-Za-z0-9-]'");
                self.peer_url_encode = true;
                self.user_agent = String::from("uTorrent/350(111258886)(44294)");
                self.connection = Some(String::from("Close"));
                self.accept_language = String::from("");
            },
            ClientVersion::Utorrent_3_5_1_44332 => {
                self.name = String::from("utorrent-3.5.1_44332");
                self.key_algorithm = crate::algorithm::Algorithm::Hash;
                self.key_uppercase = Some(true);
                self.key_refresh_on = crate::RefreshInterval::TimedOrAfterStartedAnnounce;
                self.key_refresh_every = Some(10);
                self.peer_algorithm = crate::algorithm::Algorithm::Regex;
                self.peer_pattern = String::from(r"b'-UT3515-(,\xc2\xad)[\x01-\xc3\xbf]{10}'");
                self.uppercase_encoded_hex = false;
                self.num_want = 200; self.num_want_on_stop = 0;
                self.query = String::from("info_hash={infohash}&peer_id={peerid}&port={port}&uploaded={uploaded}&downloaded={downloaded}&left={left}&corrupt=0&key={key}&event={event}&numwant={numwant}&compact=1&no_peer_id=1");
                self.encoding_exclusion_pattern = String::from(r"b'[A-Za-z0-9-]'");
                self.peer_url_encode = true;
                self.user_agent = String::from("uTorrent/351(111389996)(44332)");
                self.connection = Some(String::from("Close"));
                self.accept_language = String::from("");
            },
            ClientVersion::Utorrent_3_5_3_44358 => {
                self.name = String::from("utorrent-3.5.3_44358");
                self.key_algorithm = crate::algorithm::Algorithm::Hash;
                self.key_uppercase = Some(true);
                self.key_refresh_on = crate::RefreshInterval::TimedOrAfterStartedAnnounce;
                self.key_refresh_every = Some(10);
                self.peer_algorithm = crate::algorithm::Algorithm::Regex;
                self.peer_pattern = String::from(r"b'-UT353S-F(\xc2\xad)[\x01-\xc3\xbf]{10}'");
                self.uppercase_encoded_hex = false;
                self.num_want = 200; self.num_want_on_stop = 0;
                self.query = String::from("info_hash={infohash}&peer_id={peerid}&port={port}&uploaded={uploaded}&downloaded={downloaded}&left={left}&corrupt=0&key={key}&event={event}&numwant={numwant}&compact=1&no_peer_id=1");
                self.encoding_exclusion_pattern = String::from(r"b'[A-Za-z0-9-]'");
                self.peer_url_encode = true;
                self.user_agent = String::from("uTorrent/353(111652166)(44358)");
                self.connection = Some(String::from("Close"));
                self.accept_language = String::from("");
            },
            ClientVersion::Utorrent_3_5_3_44428 => {
                self.name = String::from("utorrent-3.5.3_44428");
                self.key_algorithm = crate::algorithm::Algorithm::Hash;
                self.key_uppercase = Some(true);
                self.key_refresh_on = crate::RefreshInterval::TimedOrAfterStartedAnnounce;
                self.key_refresh_every = Some(10);
                self.peer_algorithm = crate::algorithm::Algorithm::Regex;
                self.peer_pattern = String::from(r"b'-UT353S-(\xc2\x8c)(\xc2\xad)[\x01-\xc3\xbf]{10}'");
                self.uppercase_encoded_hex = false;
                self.num_want = 200; self.num_want_on_stop = 0;
                self.query = String::from("info_hash={infohash}&peer_id={peerid}&port={port}&uploaded={uploaded}&downloaded={downloaded}&left={left}&corrupt=0&key={key}&event={event}&numwant={numwant}&compact=1&no_peer_id=1");
                self.encoding_exclusion_pattern = String::from(r"b'[A-Za-z0-9-]'");
                self.peer_url_encode = true;
                self.user_agent = String::from("uTorrent/353(111652236)(44428)");
                self.connection = Some(String::from("Close"));
                self.accept_language = String::from("");
            },
            ClientVersion::Utorrent_3_5_4_44498 => {
                self.name = String::from("utorrent-3.5.4_44498");
                self.key_algorithm = crate::algorithm::Algorithm::Hash;
                self.key_uppercase = Some(true);
                self.key_refresh_on = crate::RefreshInterval::TimedOrAfterStartedAnnounce;
                self.key_refresh_every = Some(10);
                self.peer_algorithm = crate::algorithm::Algorithm::Regex;
                self.peer_pattern = String::from(r"b'-UT354S-(\xc3\x92)(\xc2\xad)[\x01-\xc3\xbf]{10}'");
                self.uppercase_encoded_hex = false;
                self.num_want = 200; self.num_want_on_stop = 0;
                self.query = String::from("info_hash={infohash}&peer_id={peerid}&port={port}&uploaded={uploaded}&downloaded={downloaded}&left={left}&corrupt=0&key={key}&event={event}&numwant={numwant}&compact=1&no_peer_id=1");
                self.encoding_exclusion_pattern = String::from(r"b'[A-Za-z0-9-]'");
                self.peer_url_encode = true;
                self.user_agent = String::from("uTorrent/354(111783378)(44498)");
                self.connection = Some(String::from("Close"));
                self.accept_language = String::from("");
            },
            ClientVersion::Vuze_5_7_5_0 => {
                self.name = String::from("vuze-5.7.5.0");
                self.key_algorithm = crate::algorithm::Algorithm::Regex;
                self.key_pattern = String::from(r"b'[A-Za-z0-9]{8}'");
                self.key_uppercase = None;
                self.key_refresh_on = crate::RefreshInterval::TorrentVolatile;
                self.peer_algorithm = crate::algorithm::Algorithm::Regex;
                self.peer_pattern = String::from(r"b'-AZ5750-[a-zA-Z0-9]{12}'");
                self.peer_refresh_on = crate::RefreshInterval::TorrentVolatile;
                self.uppercase_encoded_hex = true;
                self.num_want = 100; self.num_want_on_stop = 0;
                self.query = String::from("info_hash={infohash}&peer_id={peerid}&port={port}&azudp={port}&uploaded={uploaded}&downloaded={downloaded}&left={left}&corrupt=0&event={event}&numwant={numwant}&no_peer_id=1&compact=1&key={key}&azver=3");
                self.encoding_exclusion_pattern = String::from(r"b'[A-Za-z0-9_~\\\\(\\\\)\\\\!\\\\.\\\\*-]'");
                self.peer_url_encode = false;
                self.user_agent = String::from("Azureus 5.7.5.0;{os};Java {java}");
                self.connection = Some(String::from("close"));
                self.accept = String::from("text/html, image/gif, image/jpeg, *; q=.2, */*; q=.2");
                self.accept_language = String::from("");
            },
        };
        self.generate_key();
        self.generate_peer_id();
    }
}

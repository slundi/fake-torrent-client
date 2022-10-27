#[allow(non_camel_case_types)]
#[derive(Clone, Debug)]
pub enum Client {
    #[deprecated(note = "The last update was more than 2 years ago")]
    BITTORRENT_7_10_1_43917,
    #[deprecated(note = "The last update was more than 2 years ago")]
    BITTORRENT_7_10_3_44359,
    #[deprecated(note = "The last update was more than 2 years ago")]
    BITTORRENT_7_10_3_44429,
    #[deprecated(note = "The last update was more than 2 years ago")]
    DELUGE_1_3_13,
    #[deprecated(note = "The last update was more than 2 years ago")]
    DELUGE_1_3_14,
    #[deprecated(note = "The last update was more than 2 years ago")]
    DELUGE_1_3_15,
    #[deprecated(note = "The last update was more than 2 years ago")]
    DELUGE_2_0_3,
    #[deprecated(note = "The last update was more than 2 years ago")]
    LEAP_2_6_0_1,
    #[deprecated(note = "The last update was more than 2 years ago")]
    QBITTORRENT_3_3_1,
    #[deprecated(note = "The last update was more than 2 years ago")]
    QBITTORRENT_3_3_13,
    #[deprecated(note = "The last update was more than 2 years ago")]
    QBITTORRENT_3_3_14,
    #[deprecated(note = "The last update was more than 2 years ago")]
    QBITTORRENT_3_3_15,
    #[deprecated(note = "The last update was more than 2 years ago")]
    QBITTORRENT_3_3_16,
    #[deprecated(note = "The last update was more than 2 years ago")]
    QBITTORRENT_3_3_7,
    #[deprecated(note = "The last update was more than 2 years ago")]
    QBITTORRENT_4_0_0,
    #[deprecated(note = "The last update was more than 2 years ago")]
    QBITTORRENT_4_0_1,
    #[deprecated(note = "The last update was more than 2 years ago")]
    QBITTORRENT_4_0_2,
    #[deprecated(note = "The last update was more than 2 years ago")]
    QBITTORRENT_4_0_3,
    #[deprecated(note = "The last update was more than 2 years ago")]
    QBITTORRENT_4_0_4,
    #[deprecated(note = "The last update was more than 2 years ago")]
    QBITTORRENT_4_1_0,
    #[deprecated(note = "The last update was more than 2 years ago")]
    QBITTORRENT_4_1_1,
    #[deprecated(note = "The last update was more than 2 years ago")]
    QBITTORRENT_4_1_2,
    #[deprecated(note = "The last update was more than 2 years ago")]
    QBITTORRENT_4_1_3,
    #[deprecated(note = "The last update was more than 2 years ago")]
    QBITTORRENT_4_1_4,
    #[deprecated(note = "The last update was more than 2 years ago")]
    QBITTORRENT_4_1_5,
    #[deprecated(note = "The last update was more than 2 years ago")]
    QBITTORRENT_4_1_6,
    #[deprecated(note = "The last update was more than 2 years ago")]
    QBITTORRENT_4_1_7,
    #[deprecated(note = "The last update was more than 2 years ago")]
    QBITTORRENT_4_1_8,
    #[deprecated(note = "The last update was more than 2 years ago")]
    QBITTORRENT_4_1_9,
    #[deprecated(note = "The last update was more than 2 years ago")]
    QBITTORRENT_4_2_0,
    #[deprecated(note = "The last update was more than 2 years ago")]
    QBITTORRENT_4_2_1,
    #[deprecated(note = "The last update was more than 2 years ago")]
    QBITTORRENT_4_2_2,
    #[deprecated(note = "The last update was more than 2 years ago")]
    QBITTORRENT_4_2_3,
    #[deprecated(note = "The last update was more than 2 years ago")]
    QBITTORRENT_4_2_4,
    #[deprecated(note = "The last update was more than 2 years ago")]
    QBITTORRENT_4_2_5,
    #[deprecated(note = "The last update was more than 2 years ago")]
    QBITTORRENT_4_3_0_1,
    QBITTORRENT_4_3_0,
    QBITTORRENT_4_3_1,
    QBITTORRENT_4_3_2,
    QBITTORRENT_4_3_3,
    QBITTORRENT_4_3_4_1,
    QBITTORRENT_4_3_5,
    QBITTORRENT_4_3_6,
    QBITTORRENT_4_3_8,
    QBITTORRENT_4_3_9,
    QBITTORRENT_4_4_2,
    QBITTORRENT_4_4_3_1,
    RTORRENT_0_9_6_0_13_6,
    #[deprecated(note = "The last update was more than 2 years ago")]
    TRANSMISSION_2_82_14160,
    #[deprecated(note = "The last update was more than 2 years ago")]
    TRANSMISSION_2_92_14714,
    #[deprecated(note = "The last update was more than 2 years ago")]
    TRANSMISSION_2_93,
    #[deprecated(note = "The last update was more than 2 years ago")]
    TRANSMISSION_2_94,
    TRANSMISSION_3_00,
    #[deprecated(note = "The last update was more than 2 years ago")]
    UTORRENT_3_2_2_28500,
    #[deprecated(note = "The last update was more than 2 years ago")]
    UTORRENT_3_5_0_43916,
    #[deprecated(note = "The last update was more than 2 years ago")]
    UTORRENT_3_5_0_44090,
    #[deprecated(note = "The last update was more than 2 years ago")]
    UTORRENT_3_5_0_44294,
    #[deprecated(note = "The last update was more than 2 years ago")]
    UTORRENT_3_5_1_44332,
    #[deprecated(note = "The last update was more than 2 years ago")]
    UTORRENT_3_5_3_44358,
    #[deprecated(note = "The last update was more than 2 years ago")]
    UTORRENT_3_5_3_44428,
    #[deprecated(note = "The last update was more than 2 years ago")]
    UTORRENT_3_5_4_44498,
    #[deprecated(note = "The last update was more than 2 years ago")]
    VUZE_5_7_5_0,
}

pub struct UnknownClientKey;

pub fn get_client_name(key: &Client) -> Result<String, UnknownClientKey> {
    match key {
        BITTORRENT_7_10_1_43917 => Ok(String::from("bittorrent-7.10.1_43917")),
        BITTORRENT_7_10_3_44359 => Ok(String::from("bittorrent-7.10.3_44359")),
        BITTORRENT_7_10_3_44429 => Ok(String::from("bittorrent-7.10.3_44429")),
        DELUGE_1_3_13 => Ok(String::from("deluge-1.3.13")),
        DELUGE_1_3_14 => Ok(String::from("deluge-1.3.14")),
        DELUGE_1_3_15 => Ok(String::from("deluge-1.3.15")),
        DELUGE_2_0_3 => Ok(String::from("deluge-2.0.3")),
        LEAP_2_6_0_1 => Ok(String::from("leap-2.6.0.1")),
        QBITTORRENT_3_3_1 => Ok(String::from("qbittorrent-3.3.1")),
        QBITTORRENT_3_3_13 => Ok(String::from("qbittorrent-3.3.13")),
        QBITTORRENT_3_3_14 => Ok(String::from("qbittorrent-3.3.14")),
        QBITTORRENT_3_3_15 => Ok(String::from("qbittorrent-3.3.15")),
        QBITTORRENT_3_3_16 => Ok(String::from("qbittorrent-3.3.16")),
        QBITTORRENT_3_3_7 => Ok(String::from("qbittorrent-3.3.7")),
        QBITTORRENT_4_0_0 => Ok(String::from("qbittorrent-4.0.0")),
        QBITTORRENT_4_0_1 => Ok(String::from("qbittorrent-4.0.1")),
        QBITTORRENT_4_0_2 => Ok(String::from("qbittorrent-4.0.2")),
        QBITTORRENT_4_0_3 => Ok(String::from("qbittorrent-4.0.3")),
        QBITTORRENT_4_0_4 => Ok(String::from("qbittorrent-4.0.4")),
        QBITTORRENT_4_1_0 => Ok(String::from("qbittorrent-4.1.0")),
        QBITTORRENT_4_1_1 => Ok(String::from("qbittorrent-4.1.1")),
        QBITTORRENT_4_1_2 => Ok(String::from("qbittorrent-4.1.2")),
        QBITTORRENT_4_1_3 => Ok(String::from("qbittorrent-4.1.3")),
        QBITTORRENT_4_1_4 => Ok(String::from("qbittorrent-4.1.4")),
        QBITTORRENT_4_1_5 => Ok(String::from("qbittorrent-4.1.5")),
        QBITTORRENT_4_1_6 => Ok(String::from("qbittorrent-4.1.6")),
        QBITTORRENT_4_1_7 => Ok(String::from("qbittorrent-4.1.7")),
        QBITTORRENT_4_1_8 => Ok(String::from("qbittorrent-4.1.8")),
        QBITTORRENT_4_1_9 => Ok(String::from("qbittorrent-4.1.9")),
        QBITTORRENT_4_2_0 => Ok(String::from("qbittorrent-4.2.0")),
        QBITTORRENT_4_2_1 => Ok(String::from("qbittorrent-4.2.1")),
        QBITTORRENT_4_2_2 => Ok(String::from("qbittorrent-4.2.2")),
        QBITTORRENT_4_2_3 => Ok(String::from("qbittorrent-4.2.3")),
        QBITTORRENT_4_2_4 => Ok(String::from("qbittorrent-4.2.4")),
        QBITTORRENT_4_2_5 => Ok(String::from("qbittorrent-4.2.5")),
        QBITTORRENT_4_3_0_1 => Ok(String::from("qbittorrent-4.3.0.1")),
        QBITTORRENT_4_3_0 => Ok(String::from("qbittorrent-4.3.0")),
        QBITTORRENT_4_3_1 => Ok(String::from("qbittorrent-4.3.1")),
        QBITTORRENT_4_3_2 => Ok(String::from("qbittorrent-4.3.2")),
        QBITTORRENT_4_3_3 => Ok(String::from("qbittorrent-4.3.3")),
        QBITTORRENT_4_3_4_1 => Ok(String::from("qbittorrent-4.3.4.1")),
        QBITTORRENT_4_3_5 => Ok(String::from("qbittorrent-4.3.5")),
        QBITTORRENT_4_3_6 => Ok(String::from("qbittorrent-4.3.6")),
        QBITTORRENT_4_3_8 => Ok(String::from("qbittorrent-4.3.8")),
        QBITTORRENT_4_3_9 => Ok(String::from("qbittorrent-4.3.9")),
        QBITTORRENT_4_4_2 => Ok(String::from("qbittorrent-4.4.2")),
        QBITTORRENT_4_4_3_1 => Ok(String::from("qbittorrent-4.4.3.1")),
        RTORRENT_0_9_6_0_13_6 => Ok(String::from("rtorrent-0.9.6_0.13.6")),
        TRANSMISSION_2_82_14160 => Ok(String::from("transmission-2.82_14160")),
        TRANSMISSION_2_92_14714 => Ok(String::from("transmission-2.92_14714")),
        TRANSMISSION_2_93 => Ok(String::from("transmission-2.93")),
        TRANSMISSION_2_94 => Ok(String::from("transmission-2.94")),
        TRANSMISSION_3_00 => Ok(String::from("transmission-3.00")),
        UTORRENT_3_2_2_28500 => Ok(String::from("utorrent-3.2.2_28500")),
        UTORRENT_3_5_0_43916 => Ok(String::from("utorrent-3.5.0_43916")),
        UTORRENT_3_5_0_44090 => Ok(String::from("utorrent-3.5.0_44090")),
        UTORRENT_3_5_0_44294 => Ok(String::from("utorrent-3.5.0_44294")),
        UTORRENT_3_5_1_44332 => Ok(String::from("utorrent-3.5.1_44332")),
        UTORRENT_3_5_3_44358 => Ok(String::from("utorrent-3.5.3_44358")),
        UTORRENT_3_5_3_44428 => Ok(String::from("utorrent-3.5.3_44428")),
        UTORRENT_3_5_4_44498 => Ok(String::from("utorrent-3.5.4_44498")),
        VUZE_5_7_5_0 => Ok(String::from("vuze-5.7.5.0")),
        _ => Err(crate::UnknownClientKey),
    }
}

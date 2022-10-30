# fake-torrent-client

Rust library to get client information (name, peer ID, key)

## Usage

```rust
let mut client = Client::new();
client.build(ClientVersion::Qbittorrent_4_4_2);
//or tou can get the enum from a string
client.build(ClientVersion::from_str("Transmission_3_00").expect("Wrong client"));

client.get_query(); //get the query URL and HTTP headers (you have to replace fields in the url)
client.generate_key(); //generate a new key
client.generate_peer_id(); //generate a new peer ID
```

You can also get other information form the client (see doc)

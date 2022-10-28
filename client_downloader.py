"""
Downloads folders from github repo
Requires PyGithub
pip install PyGithub
"""

import base64
import csv, json
import os, datetime
from dotenv import load_dotenv, find_dotenv
from github import Github, GithubException
from github.ContentFile import ContentFile
from github.Repository import Repository

def write_algorithm(file_handler, variable_name, value):
    v = "HashNoLeadingZero"
    if value == "HASH":
        v = "Hash"
    elif value == "DIGIT_RANGE_TRANSFORMED_TO_HEX_WITHOUT_LEADING_ZEROES":
        value = "DigitRangeTransformedToHexWithoutLeadingZeroes"
    elif value == "REGEX":
        v = "Regex"
    elif value == "RANDOM_POOL_WITH_CHECKSUM":
        v = "RandomPoolWithChecksum"
    file_handler.write("                %s: crate::algorithm::Algorithm::%s,\n" % (variable_name, v))

def write_refresh_interval(file_handler, variable_name, value):
    v = "Never"
    if value == "TIMED_OR_AFTER_STARTED_ANNOUNCE":
        v = "TimedOrAfterStartedAnnounce"
    elif value == "TORRENT_PERSISTENT":
        v = "TorrentPersistent"
    elif value == "TORRENT_VOLATILE":
        v = "TorrentVolatile"
    file_handler.write("                %s: crate::RefreshInterval::%s,\n" % (variable_name, v))

def to_rust_boolean(value):
    return "true" if value else "false"

def get_sha_for_tag(repo: Repository, tag: str) -> str:
    """
    Returns a commit PyGithub object for the specified repository and tag.
    """
    branches = repo.get_branches()
    if matched_branches := [match for match in branches if match.name == tag]:
        return matched_branches[0].commit.sha

    tags = repo.get_tags()
    if matched_tags := [match for match in tags if match.name == tag]:
        return matched_tags[0].commit.sha
    else:
        raise ValueError("No Tag or Branch exists with that name")


url = "https://raw.githubusercontent.com/anthonyraymond/joal/tree/master/resources/clients"

load_dotenv(find_dotenv())

github = Github(os.environ.get("GITHUB_TOKEN", ""))
# download client files from repository
repository = github.get_repo("anthonyraymond/joal")
sha = get_sha_for_tag(repository, "master")
contents = repository.get_contents("resources/clients", ref=sha)
data = []
for content in contents:
    print(f"Processing {content.path}")
    try:
        path = content.path
        file_content = repository.get_contents(path, ref=sha)
        name = content.path.split("/")[-1][:-7]
        if not isinstance(file_content, ContentFile):
            raise ValueError("Expected ContentFile")
        if file_content.content:
            file_data = base64.b64decode(file_content.content).decode("utf-8")
            with open(f"clients/{name}.json", "w", encoding="utf-8") as out:
                out.write(file_data)
            data.append(
                (
                    name,
                    json.loads(file_data),
                )
            )
    except (GithubException, IOError, ValueError) as exc:
        print("Error processing %s: %s", content.path, exc)

columns = [
    "name",
    "kg_algo_type",
    # "kg_algo_length",  # useless, always 8
    "kg_algo_pattern",
    "kg_inclusive_lower_bound",
    "kg_inclusive_upper_bound",
    "kg_refresh_on",
    "kg_case",
    "peer_algo_type",
    "peer_pattern",
    "peer_prefix",
    "peer_refresh_on",
    "peer_url_encode",
    "url_encoder_encoding_exclusion_pattern",
    "url_encoder_encoding_case",
    "query",
    "numwant",
    "numwant_on_stop",
    "request_header_accept",
    "request_header_user_agent",
    "request_header_accept_encoding",
    "request_header_accept_language",
    "request_header_connection",
]
rows = [["" for _ in range(len(columns) + 1)] for _ in range(len(data))]
for i, c in enumerate(data):
    rows[i][0] = c[0]
    rows[i][columns.index("kg_algo_type")] = c[1]["keyGenerator"]["algorithm"]["type"]
    # if "length" in c[1]["keyGenerator"]["algorithm"]:
    #     rows[i][columns.index("kg_algo_length")] = c[1]["keyGenerator"]["algorithm"]["length"]
    if "pattern" in c[1]["keyGenerator"]["algorithm"]:
        rows[i][columns.index("kg_algo_pattern")] = c[1]["keyGenerator"]["algorithm"]["pattern"]
    if "inclusiveLowerBound" in c[1]["keyGenerator"]["algorithm"]:
        rows[i][columns.index("kg_inclusive_lower_bound")] = c[1]["keyGenerator"]["algorithm"]["inclusiveLowerBound"]
    if "inclusiveUpperBound" in c[1]["keyGenerator"]["algorithm"]:
        rows[i][columns.index("kg_inclusive_upper_bound")] = c[1]["keyGenerator"]["algorithm"]["inclusiveUpperBound"]
    rows[i][columns.index("kg_refresh_on")] = c[1]["keyGenerator"]["refreshOn"]
    rows[i][columns.index("kg_case")] = c[1]["keyGenerator"]["keyCase"]
    rows[i][columns.index("peer_algo_type")] = c[1]["peerIdGenerator"]["algorithm"]["type"]
    if "pattern" in c[1]["peerIdGenerator"]["algorithm"]:
        rows[i][columns.index("peer_pattern")] = c[1]["peerIdGenerator"]["algorithm"]["pattern"]
    if "charactersPool" in c[1]["peerIdGenerator"]["algorithm"]:
        rows[i][columns.index("peer_pattern")] = c[1]["peerIdGenerator"]["algorithm"]["charactersPool"]
    if "prefix" in c[1]["peerIdGenerator"]["algorithm"]:
        rows[i][columns.index("peer_prefix")] = c[1]["peerIdGenerator"]["algorithm"]["prefix"]
    rows[i][columns.index("peer_refresh_on")] = c[1]["peerIdGenerator"]["refreshOn"]
    rows[i][columns.index("peer_url_encode")] = c[1]["peerIdGenerator"]["shouldUrlEncode"]
    rows[i][columns.index("url_encoder_encoding_exclusion_pattern")] = c[1]["urlEncoder"]["encodingExclusionPattern"]
    rows[i][columns.index("url_encoder_encoding_case")] = c[1]["urlEncoder"]["encodedHexCase"]
    rows[i][columns.index("query")] = c[1]["query"]
    rows[i][columns.index("numwant")] = c[1]["numwant"]
    rows[i][columns.index("numwant_on_stop")] = c[1]["numwantOnStop"]
    for h in c[1]["requestHeaders"]:
        if h["name"] == "User-Agent":
            rows[i][columns.index("request_header_user_agent")] = h["value"]
        elif h["name"] == "Accept":
            rows[i][columns.index("request_header_accept")] = h["value"]
        elif h["name"] == "Accept-Encoding":
            rows[i][columns.index("request_header_accept_encoding")] = h["value"]
        elif h["name"] == "Connection":
            rows[i][columns.index("request_header_connection")] = h["value"]
        elif h["name"] == "Accept-Language":
            rows[i][columns.index("request_header_accept_language")] = h["value"]

wtr = csv.writer(open("clients.csv", "w", encoding="utf-8"), delimiter=",", lineterminator="\n")
wtr.writerow(columns)
for r in rows:
    wtr.writerow(r)

with open("src/clients.rs", "w", encoding="utf-8") as f:
    f.write("// Generated file, last update was: %s\n" % datetime.datetime.now().strftime("%Y-%m-%d %H:%M"))
    f.write("#[allow(non_camel_case_types)]\n")
    f.write("#[derive(Clone, Debug)]\npub enum ClientVersion {\n")
    for r in rows:
        f.write("    %s,\n" % r[0].title().replace(".", "_").replace("-", "_"))
    f.write("}\n\nimpl crate::Client {\n    /// Build and return the client drom the given key\n")
    f.write("    pub fn from(client_version: ClientVersion) -> crate::Client {\n")
    f.write("        let mut client = match client_version {\n")
    for r in rows:
        f.write("            ClientVersion::%s => crate::Client {\n" % r[0].title().replace(".", "_").replace("-", "_"))
        f.write("                name: String::from(\"%s\"),\n" % r[0])
        # key
        write_algorithm(f, "key_algorithm", r[columns.index("kg_algo_type")])
        # if r[columns.index("kg_algo_length")] and r[columns.index("kg_algo_length")] != "8":
        #     f.write("                key_length: %s,\n" % r[columns.index("kg_algo_length")])
        if r[columns.index("kg_algo_pattern")]:
            f.write("                key_pattern: String::from(\"%s\"),\n" % r[columns.index("kg_algo_pattern")])
        if r[columns.index("kg_case")] == "upper":
            f.write("                key_uppercase: Some(true),\n")
        elif r[columns.index("kg_case")] == "lower":
            f.write("                key_uppercase: Some(false),\n")
        else:
            f.write("                key_uppercase: None,\n")
        # if r[columns.index("kg_inclusive_lower_bound")] and r[columns.index("kg_inclusive_upper_bound")]:
        #     f.write("                TODO: String::from(\"%s\"),\n" % (r[columns.index("kg_inclusive_lower_bound")]))
        write_refresh_interval(f, "key_refresh_on", r[columns.index("kg_refresh_on")])
        # peer
        if r[columns.index("peer_algo_type")] == "REGEX":
            f.write("                peer_pattern: String::from(\"%s\"),\n" % r[columns.index("peer_pattern")].replace("\\","\\\\"))
        else:
            write_algorithm(f, "peer_algorithm", r[columns.index("peer_algo_type")])
            f.write("                peer_prefix: String::from(\"%s\"),\n" % r[columns.index("peer_prefix")])
        if r[columns.index("peer_refresh_on")] != "NEVER":
            write_refresh_interval(f, "peer_refresh_on", r[columns.index("peer_refresh_on")])
        f.write("                uppercase_encoded_hex: %s,\n" % to_rust_boolean(r[columns.index("url_encoder_encoding_case")] == "upper"))
        # misc
        f.write("                num_want: %s, num_want_on_stop: %s,\n" % (r[columns.index("numwant")], r[columns.index("numwant_on_stop")]))
        f.write("                query: String::from(\"%s\"),\n" % r[columns.index("query")])
        f.write("                encoding_exclusion_pattern: String::from(\"%s\"),\n" % r[columns.index("url_encoder_encoding_exclusion_pattern")].replace("\\","\\\\"))
        f.write("                peer_url_encode: %s,\n" % to_rust_boolean(r[columns.index("peer_url_encode")]))
        # request headers
        f.write("                user_agent: String::from(\"%s\"),\n" % r[columns.index("request_header_user_agent")])
        if r[columns.index("request_header_accept_encoding")] != "gzip":
            f.write("                accept_encoding: String::from(\"%s\"),\n" % r[columns.index("request_header_accept_encoding")])
        if r[columns.index("request_header_connection")]:
            f.write("                connection: Some(String::from(\"%s\")),\n" % r[columns.index("request_header_connection")])
        if r[columns.index("request_header_accept")]:
            f.write("                accept: String::from(\"%s\"),\n" % r[columns.index("request_header_accept")])
        if r[columns.index("request_header_accept_language")] != "gzip":
            f.write("                accept_language: String::from(\"%s\"),\n" % r[columns.index("request_header_accept_language")])
        f.write("                ..crate::Client::default()\n")
        f.write("            },\n")
    f.write("        };\n        client.generate_key();\n        client.generate_peer_id();        client\n")
    f.write("    }\n}\n")

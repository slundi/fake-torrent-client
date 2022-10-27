"""
Downloads folders from github repo
Requires PyGithub
pip install PyGithub
"""

import base64
import csv, json
import os
from dotenv import load_dotenv, find_dotenv
from itertools import chain
from github import Github, GithubException
from github.ContentFile import ContentFile
from github.Repository import Repository


def get_sha_for_tag(repository: Repository, tag: str) -> str:
    """
    Returns a commit PyGithub object for the specified repository and tag.
    """
    branches = repository.get_branches()
    if matched_branches := [match for match in branches if match.name == tag]:
        return matched_branches[0].commit.sha

    tags = repository.get_tags()
    if matched_tags := [match for match in tags if match.name == tag]:
        return matched_tags[0].commit.sha
    else:
        raise ValueError("No Tag or Branch exists with that name")

url = "https://raw.githubusercontent.com/anthonyraymond/joal/tree/master/resources/clients"

load_dotenv(find_dotenv())

github = Github(
    os.environ.get("GITHUB_TOKEN", "")
)
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
            file_data = base64.b64decode(file_content.content)
            with open(f"clients/{name}.json", "w") as out:
                out.write(json)
            data.append(
                (
                    name,
                    json.loads(file_data.decode("utf-8")),
                )
            )
    except (GithubException, IOError, ValueError) as exc:
        print("Error processing %s: %s", content.path, exc)

columns = [
    "name",
    "kg_algo_type",
    "kg_algo_length",
    "kg_algo_pattern",
    "kg_inclusive_lower_bound",
    "kg_inclusive_upper_bound",
    "kg_refresh_on",
    "kg_case",
    "peer_algo_type",
    "peer_pattern",
    "peer_refresh_on",
    "peer_url_encode",
    "url_encoder_encoding_exclusion_pattern",
    "url_encoder_encoding_lower_case",
    "query",
    "numwant",
    "numwant_on_stop",
    "request_header_accept",
    "request_header_user_agent",
    "request_header_accept_encoding",
    "request_header_connection",
]
rows = [["" for _ in range(len(columns) + 1)] for _ in range(len(data))]
for i, c in enumerate(data):
    rows[i][0] = c[0]
    rows[i][columns.index("kg_algo_type")] = c[1]["keyGenerator"]["algorithm"]["type"]
    if "length" in c[1]["keyGenerator"]["algorithm"]:
        rows[i][columns.index("kg_algo_length")] = c[1]["keyGenerator"]["algorithm"]["length"]
    if "pattern" in c[1]["keyGenerator"]["algorithm"]:
        rows[i][columns.index("kg_algo_pattern")] = c[1]["keyGenerator"]["algorithm"]["pattern"]
    if "inclusiveLowerBound" in c[1]["keyGenerator"]["algorithm"]:
        rows[i][columns.index("kg_inclusive_lower_bound")] = c[1]["keyGenerator"]["algorithm"]["inclusiveLowerBound"]
    if "inclusiveUpperBound" in c[1]["keyGenerator"]["algorithm"]:
        rows[i][columns.index("kg_inclusive_upper_bound")] = c[1]["keyGenerator"]["algorithm"]["inclusiveUpperBound"]
    rows[i][columns.index("kg_refresh_on")] = c[1]["keyGenerator"]["refreshOn"]
    rows[i][columns.index("kg_case")] = c[1]["keyGenerator"]["keyCase"]
    rows[i][columns.index("peer_algo_type")] = c[1]["peerIdGenerator"]["algorithm"]["type"]
    rows[i][columns.index("peer_pattern")] = c[1]["peerIdGenerator"]["algorithm"]["type"]
    rows[i][columns.index("peer_refresh_on")] = c[1]["peerIdGenerator"]["algorithm"]
    rows[i][columns.index("peer_url_encode")] = c[1]["peerIdGenerator"]["shouldUrlEncode"]
    rows[i][columns.index("url_encoder_encoding_exclusion_pattern")] = c[1]["urlEncoder"]["encodingExclusionPattern"]
    rows[i][columns.index("url_encoder_encoding_lower_case")] = c[1]["urlEncoder"]["encodedHexCase"]
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

wtr = csv.writer(open ("clients.csv", 'w'), delimiter=',', lineterminator='\n')
wtr.writerow(columns)
for r in rows: wtr.writerow(r)

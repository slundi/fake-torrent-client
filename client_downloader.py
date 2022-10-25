"""
Downloads folders from github repo
Requires PyGithub
pip install PyGithub
"""

import base64
import json
import os

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
        if not isinstance(file_content, ContentFile):
            raise ValueError("Expected ContentFile")
        if file_content.content:
            file_data = base64.b64decode(file_content.content)
            data.append((content.path.split('/')[-1][:-7], json.loads(file_data.decode("utf-8"))))
    except (GithubException, IOError, ValueError) as exc:
        print("Error processing %s: %s", content.path, exc)
    break

# process JSON to generate a CSV
for c in data:
    print(c)

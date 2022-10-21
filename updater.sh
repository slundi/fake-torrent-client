#!/bin/bash
# compilation of all files from https://github.com/anthonyraymond/joal/tree/master/scripts/bittorrent-client-update-detector (2022-10-21)
################### libtorrent functions ###################
version_to_char() {
  if [ $1 -ge 0 ] && [ $1 -lt 10 ]; then
    echo $1
  elif [ $1 -ge 10 ]; then
     #return char('A' + (v - 10));
     echo 0x$(( $(printf "%x" "'A'") + ($1 - 10))) | xxd -r
  fi
}

libtorrent__compute_peer_id_prefix() {
  prefix='-'
  prefix=$prefix$(echo $1 | head -c 2)
  prefix=$prefix$(version_to_char $2)
  prefix=$prefix$(version_to_char $3)
  prefix=$prefix$(version_to_char $4)
  prefix=$prefix$(version_to_char $5)
  prefix=$prefix'-'

  echo "$prefix"
}

libtorrent_get_key_format() {
  # fn de generation de la clé: https://github.com/arvidn/libtorrent/blob/master/src/torrent.cpp std::uint32_t torrent::tracker_key() const
  # Key format (uppercase) https://github.com/arvidn/libtorrent/blob/master//src/http_tracker_connection.cpp "&key=%08X" (capital X means uppercased hexa, %08 means length of 8 and left-padded with 0)
  echo "Libtorrent generate keys matching regex pattern [0-F]{8} (uppercased)"
}

################### qBittorent ###################
# clean tempSource folder
qBittorrentTempFolder="./tempSource/qBittorrent"
rm -rf $qBittorrentTempFolder
mkdir -p $qBittorrentTempFolder


if [ -z ${1+x} ]; then
  # Download latest release
  tarballUrl=$(curl -s https://api.github.com/repos/qbittorrent/qBittorrent/tags \
    | grep "tarball_url" \
    | head -1 \
    | cut -d : -f 2,3 \
    | cut -d , -f 1 \
    | tr -d \")
else
  # download the release from the "tarball_url" given in parameter (see https://api.github.com/repos/qbittorrent/qBittorrent/tags)
  tarballUrl=$1
fi
curl -L $tarballUrl --output $qBittorrentTempFolder/qBittorrent.tar.gz

# uncompress the archive
tar -xzf $qBittorrentTempFolder/qBittorrent.tar.gz -C $qBittorrentTempFolder/ --strip 1

# seach for qBittorent versions
VER_MAJOR=$(grep "#define QBT_VERSION_MAJOR " $qBittorrentTempFolder/src/base/version.h.in | cut -d ' ' -f 3 | tr -d '[:space:]')
VER_MINOR=$(grep "#define QBT_VERSION_MINOR " $qBittorrentTempFolder/src/base/version.h.in | cut -d ' ' -f 3 | tr -d '[:space:]')
VER_BUGFIX=$(grep "#define QBT_VERSION_BUGFIX " $qBittorrentTempFolder/src/base/version.h.in | cut -d ' ' -f 3 | tr -d '[:space:]')
VER_BUILD=$(grep "#define QBT_VERSION_BUILD " $qBittorrentTempFolder/src/base/version.h.in | cut -d ' ' -f 3 | tr -d '[:space:]')
VER_STATUS=$(grep "#define QBT_VERSION_STATUS " $qBittorrentTempFolder/src/base/version.h.in | cut -d ' ' -f 3 | cut -d '"' -f 1 | tr -d '[:space:]')

PROJECT_VERSION="${VER_MAJOR}.${VER_MINOR}.${VER_BUGFIX}"
if [ $VER_BUILD -ne '0' ]; then
  PROJECT_VERSION="${PROJECT_VERSION}.${VER_BUILD}"
fi
PROJECT_VERSION="${PROJECT_VERSION}${VER_STATUS}"

QBT_VERSION_MAJOR=${VER_MAJOR}
QBT_VERSION_MINOR=${VER_MINOR}
QBT_VERSION_BUGFIX=${VER_BUGFIX}
QBT_VERSION_BUILD=${VER_BUILD}
QBT_VERSION="v${PROJECT_VERSION}"
QBT_VERSION_2="${PROJECT_VERSION}"

# extract user agent
non_expanded_user_agent=$(grep "USER_AGENT\[\] =" $qBittorrentTempFolder/src/base/bittorrent/session.cpp | cut -d '=' -f 2 | tr -d '[:space:]' | tr -d '[";]' | sed -e 's/QBT_VERSION/$QBT_VERSION/g')
user_agent=$(eval echo "$non_expanded_user_agent")
echo "User-Agent is: $user_agent"


# extract beginning of peer_id
bt_peer_id_small_name=$(grep "PEER_ID\[\] =" $qBittorrentTempFolder/src/base/bittorrent/session.cpp | cut -d '=' -f 2 | tr -d '[:space:]' | tr -d '[";]')

if [ $(grep -c "lt::generate_fingerprint(PEER_ID, QBT_VERSION_MAJOR, QBT_VERSION_MINOR, QBT_VERSION_BUGFIX, QBT_VERSION_BUILD);" $qBittorrentTempFolder/src/base/bittorrent/session.cpp) -lt 1 ]; then
  echo "WHHHHHOOOOPS, the peerid prefix generator might have changed."
  exit 1
fi

peer_id_prefix=$(libtorrent__compute_peer_id_prefix "$bt_peer_id_small_name" "$QBT_VERSION_MAJOR" "$QBT_VERSION_MINOR" "$QBT_VERSION_BUGFIX" "$QBT_VERSION_BUILD")
echo "Peer_id prefix is: $peer_id_prefix"

echo "key : qBittorent is using libtorrent => $(libtorrent_get_key_format)"


# clean tempSource folder
rm -rf $qBittorrentTempFolder

################### Transmission ###################
transmissionTempFolder="./tempSource/transmission"
rm -rf $transmissionTempFolder
mkdir -p $transmissionTempFolder


if [ -z ${1+x} ]; then
  # Download latest release
  tarballUrl=$(curl -s https://api.github.com/repos/transmission/transmission/releases/latest \
    | grep "tarball_url" \
    | head -1 \
    | cut -d : -f 2,3 \
    | cut -d , -f 1 \
    | tr -d \")
else
  # download the release from the "tarball_url" given in parameter (see https://api.github.com/repos/transmission/transmission/releases)
  tarballUrl=$1
fi
curl -L $tarballUrl --output $transmissionTempFolder/transmission.tar.gz

# uncompress the archive
tar -xzf $transmissionTempFolder/transmission.tar.gz -C $transmissionTempFolder/ --strip 1


if [ $(grep -c '(e, CURLOPT_USERAGENT, TR_NAME "/" SHORT_VERSION_STRING);' $transmissionTempFolder/libtransmission/web.c) -lt 1 ]; then
  echo "WHHHHHOOOOPS, the user agent generator might have changed."
  exit 1
fi
userAgentPattern=$(grep "CURLOPT_USERAGENT" $transmissionTempFolder/libtransmission/web.c)
userAgentPattern=${userAgentPattern##*,} # get text after last comma
userAgentPattern=$(echo $userAgentPattern | sed 's# "/" #/#g' | sed 's/);//g') # remove double quotes and spaces separators

TR_NAME=$(grep "TR_NAME" $transmissionTempFolder/libtransmission/session.h | cut -d'"' -f 2) # Get the value between quotes

if [ $(grep -c '#define SHORT_VERSION_STRING      "${TR_USER_AGENT_PREFIX}"' $transmissionTempFolder/libtransmission/version.h.in) -lt 1 ]; then
  echo "WHHHHHOOOOPS, the user agent SHORT_VERSION_STRING might have changed."
  exit 1
fi
SHORT_VERSION_STRING=$(grep "set(TR_USER_AGENT_PREFIX" $transmissionTempFolder/CMakeLists.txt | cut -d'"' -f 2) # Get the value between quotes


userAgent=$(echo $userAgentPattern | sed "s/TR_NAME/${TR_NAME}/g" |sed "s/SHORT_VERSION_STRING/${SHORT_VERSION_STRING}/g")
echo "User-Agent is: $userAgent"


peer_id_prefix_expr=$(grep "peer_id_prefix=" $transmissionTempFolder/update-version-h.sh | sed 's/peer_id_prefix=//g' | sed 's|configure.ac|$transmissionTempFolder/configure.ac|')

peer_id_prefix=$(eval echo "$peer_id_prefix_expr")
echo "Peer_id prefix is: $peer_id_prefix"



echo "key : An int between 1 and 2147483647 (inclusive) which is converted to hex (without leading zero)"


# clean tempSource folder
rm -rf $transmissionTempFolder

# fn du user-agent : https://github.com/transmission/transmission/blob/master/libtransmission/web.c static CURL* createEasy(tr_session* s, struct tr_web* web, struct tr_web_task* task) => curl_easy_setopt(e, CURLOPT_USERAGENT, TR_NAME "/" SHORT_VERSION_STRING);

# fn de generation des peerid : https://github.com/transmission/transmission/blob/eb5d1a79cbe1b9bc5b22fdcc598694ecd4d02f43/libtransmission/session.c > void tr_peerIdInit(uint8_t* buf)

# fn de generation des key : https://github.com/transmission/transmission/blob/a86266d3c29f6a5b4103d9c3d60e10165d410226/libtransmission/announcer.c > void tr_announcerInit(tr_session* session)

# fn de creation des url d'announce : https://github.com/transmission/transmission/blob/c11f2870fd18ff781ca06ce84b6d43541f3293dd/libtransmission/announcer-http.c static char* announce_url_new(tr_session const* session, tr_announce_request const* req)

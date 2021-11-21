#!/usr/bin/env bash
set -ueo pipefail

firefox_dir="$HOME/Library/Application Support/Firefox"
if [ ! -d "$firefox_dir" ]; then
	echo "cannot search for firefox cookies in non-existing directory '${firefox_dir}'" >> /dev/stderr
	exit 1
fi

firefox_cookies=$(find "$firefox_dir" -type f -name cookies.sqlite | head -n 1)
if [ ! -f "$firefox_cookies" ]; then
	echo "unable to find firefox cookies in '${firefox_dir}'" >> /dev/stderr
	exit 1
fi

get_session_value="
	select value from moz_cookies
	where host = '.adventofcode.com' and name = 'session';
"
token=$(sqlite3 "$firefox_cookies" "$get_session_value")
if [ -z "$token" ]; then
	echo unable to find session cookie for host adventofcode.com
	echo log in with firefox to adventofcode.com, then try again
	exit 1
fi
outpath="$(dirname $0)/session_token"
echo $token > "$outpath"
chmod 600 "$outpath"
echo "ok: $outpath"

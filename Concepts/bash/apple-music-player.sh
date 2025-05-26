#!/usr/bin/env bash

# export LC_ALL=en_US.UTF-8
#
# current_dir="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
# source $current_dir/utils.sh


getTrack() {
	local track_info
	track_info=$(osascript <<'END'
		tell application "System Events"
			if not (exists process "Music") then return "not running"
		end tell
		tell application "Music"
			if player state is stopped then return "stopped"
			set artistName to artist of current track
			set trackName to name of current track
			set t to artistname & " - " & trackName 
			return t 
		end tell
END
	)
	case "$track_info" in 
		"not running") echo "Music is not running" ;;
		"stopped") echo "Music stopped" ;;
		*) echo "$track_info" ;;
	esac

}



main() {
	local cache_file="/tmp/tmux_amp_cache"
	# RATE=$(get_tmux_option "@dracula-refresh-rate" 5)

	getTrack > "$cache_file"
	cat "$cache_file"
}

main "$@"

tell application "Spotify"
    if player state is stopped then return "stopped"
	set t to "♪ " & artist of current track & " - " & name of current track
    if player state is paused then set t to "❚❚ " & artist of current track & " - " & name of current track
    display dialog t
end tell

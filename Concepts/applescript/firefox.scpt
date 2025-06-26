tell application "System Events"
	tell process "Firefox"
		set frontmost to true
		get name of front window -- May return the tab title
	end tell
end tell

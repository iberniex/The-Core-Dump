on browserPlayer(browserName)
	if (running of application "Safari") and (browserName is "Safari") then
		tell application "Safari"
			set currentTab to the front document
			set currentURL to URL of currentTab
			set pageTitle to name of currentTab
		end tell
	else if (running of application "Google Chrome") and (browserName is "Google Chrome") then
		tell application "Google Chrome"
			set currentTab to active tab of front window
			--- set currentTab to active tab of front window
			set currentURL to URL of currentTab
			set pageTitle to title of currentTab
		end tell
	else
		return "Not supported"
	end if
	
	
	-- Check if it's a YouTube video page
	if currentURL contains "youtube.com/watch" then
		-- YouTube video titles usually follow this format: "Artist - Track Name"
		set AppleScript's text item delimiters to " - "
		set titleParts to text items of pageTitle
		
		if (count of titleParts) ≥ 2 then
			set artistName to item 1 of titleParts
			set trackName to item 2 of titleParts
			--- display dialog "Artist: " & artistName & return & "Track: " & trackName
			return artistName & " - " & trackName
		else
			--- display dialog "Could not parse artist and track from: " & pageTitle
			return "not playing"
		end if
		-- Check if it's a Spotify video page
	else if currentURL contains "open.spotify.com" then
		-- YouTube video titles usually follow this format: "Artist - Track Name"
		set AppleScript's text item delimiters to " • "
		set titleParts to text items of pageTitle
		
		if (count of titleParts) ≥ 2 then
			set artistName to item 1 of titleParts
			set trackName to item 2 of titleParts
			--- display dialog "Artist: " & artistName & return & "Track: " & trackName
			return artistName & " - " & trackName
		else
			--- display dialog "Could not parse artist and track from: " & pageTitle
			return "not playing"
		end if
		
		
	else
		display dialog "No active YT Tab"
	end if
	
end browserPlayer


browserPlayer("Google Chrome")


property playerList : {"Spotify", "Music", "Tidal", "Safari", "Google Chrome", "Firefox"}

on detectPlayer()
	repeat with appName in playerList
		set currentApp to contents of appName
		tell application "System Events"
			if (exists process currentApp) then
				log currentApp
				set status to my getPlayerStatus(currentApp)
				log status
			end if
		end tell
	end repeat
end detectPlayer

on getPlayerStatus(appName)
	try
		if appName is "Spotify" then
			tell application "Spotify"
				if player state is playing then
					return "playing"
				else if player state is paused then
					return "paused"
				else
					return "stopped"
				end if
			end tell
		else
			return "unsupported player"
		end if
	on error errMsg
		return "error: " & errMsg
	end try
end getPlayerStatus

detectPlayer()


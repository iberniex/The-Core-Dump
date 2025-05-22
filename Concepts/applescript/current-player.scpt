property playerList : {"Spotify", "Music", "Tidal", "Safari", "Google Chrome", "Firefox"}
property nativePlayerList : { "Spotify", "Music", "Tidal" }

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
		set native_current_state to my getNativeCurrentState(appName)
		return native_current_state
	on error errMsg
		return "error: " & errMsg
	end try
end getPlayerStatus

on getNativeCurrentState(appName)
	if appName is in nativePlayerList then
		set code to "tell application \"" & appName & "\" to get player state"
		set current_state to run script code
		return current_state
	else
		return "not supported"
	end if
end getNativeCurrentState

detectPlayer()

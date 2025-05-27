tell application "Arc"
	activate
	set currentTab to active tab of front window
	set tabTitle to title of currentTab
	set tabURL to URL of currentTab
end tell

return "Now playing: " & tabTitle & " (" & tabURL & ")"

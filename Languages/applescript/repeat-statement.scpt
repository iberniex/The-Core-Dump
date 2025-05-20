set wordList to words in "Ommpa lumpa"
repeat with currentWord in wordList
	log currentWord
	if contents of currentWord is equal to "lumpa" then
		display dialog "I found it!!"
	end if
end repeat

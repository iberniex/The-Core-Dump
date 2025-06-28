greeting = "Hello"

# learning on the pattern matching concept
# pin operator use 
# Instead of using the following
"""
# This should throw a warning saying that greeting doesn't match the greeting variable
"hello" = greeting
^greeting -> checks for the left side without using the equal operator.
"""

greet = fn
  ^greeting, value -> "Hi, #{value}"
  greeting, value -> "#{greeting}, #{value}"
end

IO.puts(greet.("Hello", "Johny"))
IO.puts(greet.("Greetings", "Johny"))

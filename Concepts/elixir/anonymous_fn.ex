# demonstrating anonymous functions

minus_3 = fn x -> x - 3 end

IO.inspect(minus_3.(123))

min = &(&1 - &2)

IO.inspect(min.(45, 22))

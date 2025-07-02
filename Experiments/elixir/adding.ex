defmodule Adding do
  def adding_three(numbers) do
    IO.inspect(Enum.map(numbers, fn x -> x + 3 end))
  end
end

list = [1, 2, 3, 4]

IO.puts(Adding.adding_three(list))

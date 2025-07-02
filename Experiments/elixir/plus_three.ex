defmodule Adding3 do
  def plus_three([]), do: []

  # def plus_three([head | tail]) do
  #   [head + 3 | tail]
  # end

  def plus_three(numbers) when is_list(numbers) do
    [head | tail] = numbers

    h = plus_three(head)
    t = plus_three(tail)
    [h | t]
  end

  def plus_three(number) when is_number(number), do: number + 3
end

list = [1, 2, 3, 4]

# IO.inspect([] ++ [1 + 3])
IO.inspect(Adding3.plus_three(list))

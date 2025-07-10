defmodule Partlist do
  def part_list(a), do: _part_list(a, [], [])

  defp _part_list([head | tail], head_store, final_store) do
    if Enum.count(tail) == 0 do
      final_store |> Enum.reverse()
    else
      head_store = [head | head_store]

      final_store = [
        [head_store |> Enum.reverse() |> Enum.join(" "), Enum.join(tail, " ")] | final_store
      ]

      _part_list(tail, head_store, final_store)
    end
  end
end

defmodule Permutations do
  def part_list(a) do
    1..(length(a) - 1)
    |> Enum.map(fn i ->
      {left, right} = Enum.split(a, i)
      [Enum.join(left, " "), Enum.join(right, " ")]
    end)
  end
end

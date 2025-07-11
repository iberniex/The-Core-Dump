defmodule Seven do
  def seven(m) do
    IO.puts(m)
    _seven(m, 0)
  end

  defp _seven(m, count) when m < 100, do: [m, count]

  defp _seven(m, count) do
    digits = Integer.digits(m)
    {last, rest} = List.pop_at(digits, -1)
    new_m = Integer.undigits(rest) - 2 * last
    _seven(new_m, count + 1)
  end
end

defmodule Seven_clean do
  def seven(m), do: _seven(m, 0)
  defp _seven(m, n) when m < 100, do: [m, n]
  defp _seven(m, n), do: _seven(div(m, 10) - rem(m, 10) * 2, n + 1)
end

defmodule MySigils do
  def sigil_u(string, []), do: String.upcase(string)

  def sigil_REX(string, []), do: Regex.match?(~r"^([\w\.]*)@([\w\.]*)", string)
end

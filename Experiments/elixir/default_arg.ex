defmodule Greeter do
  def hello(name, language_code \\ "en")

  def hello(name, language_code) when is_list(name) do
    names = Enum.join(name, ", ")

    hello(names, language_code)
  end

  def hello(name, language_code) when is_binary(name) do
    phrase(language_code) <> name
  end

  defp phrase("en"), do: "Hello, "
  defp phrase("es"), do: "Hola, "
end

IO.inspect(Greeter.hello(["Sean", "Steve"]))

IO.inspect(Greeter.hello(["Bernie", "Steve"], "es"))

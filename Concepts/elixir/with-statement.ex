import Integer

m = %{a: 4, b: 2}

odd_checker =
  with {:ok, number} <- Map.fetch(m, :a),
       true <-
         is_even(number) do
    IO.puts("#{number} divided by 2 is #{div(number, 2)}")
    :even
  else
    :error ->
      IO.puts("We don't have this item king")
      :error

    _ ->
      IO.puts("its odd")
      :odd
  end

odd_checker

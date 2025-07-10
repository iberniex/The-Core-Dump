defmodule Solution do
  def nb_year(p0, percent, aug, p) do
    do_nb_year(p0, percent, aug, p, 0)
  end

  defp do_nb_year(p0, percent, aug, p, years) do
    if p0 >= p do
      years
    else
      new_p0 = Kernel.trunc(p0 + p0 * (percent / 100) + aug)
      do_nb_year(new_p0, percent, aug, p, years + 1)
    end
  end
end

defmodule User do
  @derive {Inspect, only: [:name]}
  defstruct name: nil, roles: []
end

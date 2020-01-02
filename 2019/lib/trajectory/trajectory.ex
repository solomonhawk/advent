defmodule Trajectory do
  alias Trajectory.OrbitMap

  def parse(input) do
    input
    |> String.replace("\n", ",")
    |> String.replace(" ", "")
    |> String.split(",", trim: true)
    |> Enum.map(fn
      "COM)" <> id -> {:com, id}
      rule -> String.split(rule, ")") |> List.to_tuple()
    end)
  end

  def checksum(%OrbitMap{} = map) do
    OrbitMap.count_orbits(map)
  end

  def transfers(%OrbitMap{} = map, from, to) do
    OrbitMap.orbital_transfers(map, from, to)
  end
end

defmodule Wire.Fixer do
  def uncross({wire1, wire2}) do
    to_positions(wire1)
    |> MapSet.intersection(to_positions(wire2))
    |> Enum.map(&(distance({0, 0}, &1)))
    |> Enum.sort
    |> List.first
  end

  def distance({x1, y1}, {x2, y2}) do
    abs(x1 - x2) + abs(y1 - y2)
  end

  def to_positions(wire) do
    {result, _} = Enum.reduce(wire, {MapSet.new, {0, 0}}, fn segment, {set, {x, y}} ->
      {start_x, start_y, end_x, end_y} = case segment do
        {:right, n} -> {x + 1, y, x + n, y}
        {:left,  n} -> {x - 1, y, x - n, y}
        {:up,    n} -> {x, y + 1, x, y + n}
        {:down,  n} -> {x, y - 1, x, y - n}
      end

      visited = for x <- start_x..end_x,
                    y <- start_y..end_y,
                    into: set,
                    do: {x, y}

      {visited, {end_x, end_y}}
    end)

    result
  end
end

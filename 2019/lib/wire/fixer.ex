defmodule Wire.Fixer do
  def uncross({wire1, wire2}) do
    to_set(wire1)
    |> MapSet.intersection(to_set(wire2))
    |> Enum.map(&(distance({0, 0}, &1)))
    |> Enum.sort
    |> List.first
  end

  def uncross_min_distance({wire1, wire2}) do
    # calculate the locations visited and distance at each step
    w1 = to_map(wire1)
    w2 = to_map(wire2)

    m1 = MapSet.new(Map.keys(w1))
    m2 = MapSet.new(Map.keys(w2))

    # find the intersections
    MapSet.intersection(m1, m2)
    |> Enum.map(fn point ->
      # for each intersection, sum the distances for each wire
      Map.get(w1, point) + Map.get(w2, point)
    end)
    # find the lowest distance
    |> Enum.sort
    |> List.first
  end

  def distance({x1, y1}, {x2, y2}) do
    abs(x1 - x2) + abs(y1 - y2)
  end

  def to_set(wire) do
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

  def to_map(wire) do
    to_map(wire, Map.new, {0, 0}, 0)
  end

  def to_map([{:right, n} | rest], map, {sx, y}, d) do
    new_map = for x <- sx + 1..sx + n, into: map, do: {{x, y}, d + (x - sx)}

    to_map(rest, new_map, {sx + n, y}, d + n)
  end

  def to_map([{:left, n} | rest], map, {sx, y}, d) do
    new_map = for x <- sx - 1..sx - n, into: map, do: {{x, y}, d - (x - sx)}

    to_map(rest, new_map, {sx - n, y}, d + n)
  end

  def to_map([{:up, n} | rest], map, {x, sy}, d) do
    new_map = for y <- sy + 1..sy + n, into: map, do: {{x, y}, d + y - sy}

    to_map(rest, new_map, {x, sy + n}, d + n)
  end

  def to_map([{:down, n} | rest], map, {x, sy}, d) do
    new_map = for y <- sy - 1..sy - n, into: map, do: {{x, y}, d - (y - sy)}

    to_map(rest, new_map, {x, sy - n}, d + n)
  end

  def to_map([], map, _, _) do
    map
  end
end

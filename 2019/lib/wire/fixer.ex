defmodule Wire.Fixer do
  def uncross({wire1, wire2}) do
    to_set(wire1)
    |> MapSet.intersection(to_set(wire2))
    |> Enum.map(&distance({0, 0}, &1))
    |> Enum.sort()
    |> List.first()
  end

  def uncross_min_distance({wire1, wire2}) do
    m1 = to_map(wire1)
    m2 = to_map(wire2)

    MapSet.new(Map.keys(m1))
    |> MapSet.intersection(MapSet.new(Map.keys(m2)))
    |> Enum.map(&(Map.get(m1, &1) + Map.get(m2, &1)))
    |> Enum.sort()
    |> List.first()
  end

  def distance({x1, y1}, {x2, y2}) do
    abs(x1 - x2) + abs(y1 - y2)
  end

  def to_set(wire) do
    to_set(MapSet.new(), wire, {0, 0})
  end

  def to_set(set, [{:right, n} | rest], {sx, y}) do
    for(x <- (sx + 1)..(sx + n), into: set, do: {x, y}) |> to_set(rest, {sx + n, y})
  end

  def to_set(set, [{:left, n} | rest], {sx, y}) do
    for(x <- (sx - 1)..(sx - n), into: set, do: {x, y}) |> to_set(rest, {sx - n, y})
  end

  def to_set(set, [{:up, n} | rest], {x, sy}) do
    for(y <- (sy + 1)..(sy + n), into: set, do: {x, y}) |> to_set(rest, {x, sy + n})
  end

  def to_set(set, [{:down, n} | rest], {x, sy}) do
    for(y <- (sy - 1)..(sy - n), into: set, do: {x, y}) |> to_set(rest, {x, sy - n})
  end

  def to_set(set, [], _) do
    set
  end

  def to_map(wire) do
    to_map(Map.new(), wire, {0, 0}, 0)
  end

  def to_map(map, [{:right, n} | rest], {sx, y}, d) do
    for(x <- (sx + 1)..(sx + n), into: map, do: {{x, y}, d + (x - sx)})
    |> to_map(rest, {sx + n, y}, d + n)
  end

  def to_map(map, [{:left, n} | rest], {sx, y}, d) do
    for(x <- (sx - 1)..(sx - n), into: map, do: {{x, y}, d - (x - sx)})
    |> to_map(rest, {sx - n, y}, d + n)
  end

  def to_map(map, [{:up, n} | rest], {x, sy}, d) do
    for(y <- (sy + 1)..(sy + n), into: map, do: {{x, y}, d + (y - sy)})
    |> to_map(rest, {x, sy + n}, d + n)
  end

  def to_map(map, [{:down, n} | rest], {x, sy}, d) do
    for(y <- (sy - 1)..(sy - n), into: map, do: {{x, y}, d - (y - sy)})
    |> to_map(rest, {x, sy - n}, d + n)
  end

  def to_map(map, [], _, _) do
    map
  end
end

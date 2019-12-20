defmodule Wire.Fixer do
  @padding 2

  def uncross({wire1, wire2}, debug \\ false) do
    positions =
      Map.new
      |> to_positions(wire1)
      |> to_positions(wire2)

    if debug == true do
      bounds_with_padding(positions) |> draw_wires(positions)
    end

    positions
    |> Map.to_list
    |> Enum.filter(&(elem(&1, 1) >= 2))
    |> IO.inspect
    |> Enum.map(&(distance({0, 0}, elem(&1, 0))))
    |> Enum.sort
    |> List.first
  end

  def distance({x1, y1}, {x2, y2}) do
    abs(x1 - x2) + abs(y1 - y2)
  end

  def to_positions(map, wire) do
    {result, _} = Enum.reduce(wire, {map, {0, 0}}, fn segment, {map, {x, y}} ->
      {start_x, start_y, end_x, end_y} = case segment do
        {:right, n} -> {x + 1, y, x + n, y}
        {:left,  n} -> {x - 1, y, x - n, y}
        {:up,    n} -> {x, y + 1, x, y + n}
        {:down,  n} -> {x, y - 1, x, y - n}
      end

      visited = Enum.reduce(start_x..end_x, map, fn x, acc ->
        Enum.reduce(start_y..end_y, acc, fn y, acc ->
          Map.update(acc, {x, y}, 1, &(&1 + 1))
        end)
      end)

      {visited, {end_x, end_y}}
    end)

    result
  end

  def bounds_with_padding(positions) do
    {{x1, y1}, {x2, y2}} = positions
    |> Map.keys
    |> Enum.reduce({{0, 0}, {0, 0}}, fn {x, y}, {{x1, y1}, {x2, y2}} ->
      min_x = min(x, x1) |> min(x2)
      max_x = max(x, x1) |> max(x2)
      min_y = min(y, y1) |> min(y2)
      max_y = max(y, y1) |> max(y2)

      # bottom left corner, top right corner
      {{min_x, min_y}, {max_x, max_y}}
    end)

    {x1 - @padding..x2 + @padding, y1 - @padding..y2 + @padding}
  end

  def draw_wires({xr, yr}, positions) do
    positions = Map.put(positions, {0, 0}, :start)

    Enum.each(Enum.reverse(yr), fn y ->
      line = Enum.map(xr, fn x ->
        case Map.get(positions, {x, y}) do
          1 -> "."
          2 -> "x"
          :start -> "o"
          _ -> " "
        end
      end) |> Enum.join("")

      IO.puts line
    end)
  end
end

defmodule Wire.Parser do
  def parse_wires(input) do
    input
    |> String.split("\n", trim: true)
    |> Enum.map(&parse_wire/1)
    |> List.to_tuple
  end

  def parse_wire(input) do
    input
    |> String.split(",", trim: true)
    |> Enum.map(&parse_segment/1)
  end

  defp parse_segment("R" <> distance), do: {:right, Integer.parse(distance) |> elem(0)}
  defp parse_segment("L" <> distance), do: {:left, Integer.parse(distance) |> elem(0)}
  defp parse_segment("D" <> distance), do: {:down, Integer.parse(distance) |> elem(0)}
  defp parse_segment("U" <> distance), do: {:up, Integer.parse(distance) |> elem(0)}
end

defmodule Day5 do
  def parse do
    File.read!(Path.join(__DIR__, "input.txt"))
    |> String.split(",", trim: true)
    |> Enum.map(&(Integer.parse(&1) |> elem(0)))
  end
end

defmodule Day1.Part1 do
  def run do
    parse()
    |> Enum.map(&FuelMass.Calculator.calculate/1)
    |> Enum.sum
    |> Kernel.trunc
  end

  def parse do
    File.read!(Path.join(__DIR__, "../input.txt"))
    |> String.split("\n", trim: true)
    |> Enum.map(&(Integer.parse(&1) |> elem(0)))
  end
end

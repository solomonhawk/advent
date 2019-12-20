defmodule Day2.Part2 do
  # which input modifications to position 1 and 2 result in
  # the the output of 19690720 at position 0 (given input.txt)
  def run do
    input = parse()

    # try all combinations of pairs of numbers within the valid input range
    {i1, i2} = Enum.reduce(0..99, {0, 0}, fn x, acc ->
      Enum.reduce(0..99, acc, fn y, acc ->
        result = input
        |> List.replace_at(1, x)
        |> List.replace_at(2, y)
        |> Intcode.Fixer.fix()
        |> hd()

        case result do
          19690720 -> {x, y}
          _        -> acc
        end
      end)
    end)

    100 * i1 + i2
  end

  def parse do
    File.read!(Path.join(__DIR__, "../input.txt"))
    |> String.split(",", trim: true)
    |> Enum.map(&(Integer.parse(&1) |> elem(0)))
  end
end

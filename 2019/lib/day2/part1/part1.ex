defmodule Day2.Part1 do
  def run do
    parse()
    |> List.replace_at(1, 12) # replace the value at position 1 with 12
    |> List.replace_at(2, 2)  # replace the value at position 2 with 2
    |> Intcode.Fixer.fix()    # execute the program
    |> hd()                   # output the first value
  end

  def parse do
    File.read!(Path.join(__DIR__, "../input.txt"))
    |> String.split(",", trim: true)
    |> Enum.map(&(Integer.parse(&1) |> elem(0)))
  end
end

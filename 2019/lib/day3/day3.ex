defmodule Day3 do
  def parse do
    File.read!(Path.join(__DIR__, "input.txt"))
    |> Wire.Parser.parse_wires()
  end
end

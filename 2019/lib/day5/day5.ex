defmodule Day5 do
  def parse do
    File.read!(Path.join(__DIR__, "input.txt"))
    |> Intcode.Program.parse()
  end
end

defmodule Day7 do
  def read do
    File.read!(Path.join(__DIR__, "input.txt"))
    |> Intcode.Program.parse()
  end
end

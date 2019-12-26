defmodule Intcode.Program do
  def parse(program_text) do
    program_text
    |> String.replace("\n", "")
    |> String.replace(" ", "")
    |> String.split(",", trim: true)
    |> Enum.map(&(Integer.parse(&1) |> elem(0)))
  end

  def value_at(program, instr_pointer) do
    Enum.at(program, instr_pointer)
  end
end

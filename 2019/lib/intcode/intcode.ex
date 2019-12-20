defmodule Intcode do
  require Intcode.Constants

  alias Intcode.Constants

  def memory(instructions) do
    instructions
  end

  def instruction_at(memory, instr_pointer) do
    memory
    |> Enum.drop(instr_pointer)
    |> Enum.take(Constants.instruction_len)
  end

  def value_at(memory, instr_pointer) do
    Enum.at(memory, instr_pointer)
  end
end

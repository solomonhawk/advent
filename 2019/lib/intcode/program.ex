defmodule Intcode.Program do
  def value_at(program, instr_pointer) do
    Enum.at(program, instr_pointer)
  end
end

defmodule Intcode.Fixer do
  require Intcode.Constants
  require Intcode.Constants.OpCodes

  alias Intcode.Constants
  alias Intcode.Constants.OpCodes

  import Intcode, only: [instruction_at: 2, value_at: 2]

  def fix(memory), do: fix(memory, 0)

  def fix(memory, instr_pointer) do
    memory
    |> instruction_at(instr_pointer)
    |> execute_instruction({instr_pointer, memory})
  end

  # add
  def execute_instruction([op, p1, p2, p3], {instr_pointer, memory}) when op == OpCodes.add() do
    memory
    |> List.replace_at(p3, value_at(memory, p1) + value_at(memory, p2))
    |> fix(instr_pointer + Constants.instruction_len())
  end

  # multiply
  def execute_instruction([op, p1, p2, p3], {instr_pointer, memory}) when op == OpCodes.mult() do
    memory
    |> List.replace_at(p3, value_at(memory, p1) * value_at(memory, p2))
    |> fix(instr_pointer + Constants.instruction_len())
  end

  # halt
  def execute_instruction([op | _], {_instr_pointer, memory}) when op == OpCodes.halt() do
    memory
  end
end

defmodule Intcode.ExecutionContext do
  defstruct program: [], instruction_pointer: 0

  alias __MODULE__

  def program(%ExecutionContext{program: program}), do: program

  def instruction_pointer(%ExecutionContext{instruction_pointer: instruction_pointer}),
    do: instruction_pointer

  def new(program, instruction_pointer) do
    %ExecutionContext{
      program: program,
      instruction_pointer: instruction_pointer
    }
  end
end

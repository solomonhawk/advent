defmodule Intcode.Instruction do
  defstruct op: nil, parameters: []

  import Intcode.Constants, only: [instruction_length: 1]

  alias __MODULE__
  alias Intcode.Instruction.Parameter

  def new(op: op, parameters: parameters) do
    [instruction, modes] = parse_modes(op)

    %Instruction{
      op: instruction,
      parameters: to_params(parameters, modes)
    }
  end

  def instruction_at(program, instr_pointer) do
    program = Enum.drop(program, instr_pointer)
    op = Instruction.instruction_op(hd(program))
    command = Enum.take(program, instruction_length(op))

    Instruction.new(op: op, parameters: tl(command))
  end

  def parse_modes(op) do
    [instruction_op(op), floor(op / 100) |> pad_left(0, 3)]
  end

  def instruction_op(op), do: rem(op, 100)

  def to_params(parameters, modes) do
    Enum.zip(parameters, modes) |> Enum.map(&Parameter.to_param/1)
  end

  def pad_left(value, padder, count) when is_integer(value) do
    value
    |> Integer.digits()
    |> pad_left(padder, count)
  end

  def pad_left(value, _, count) when is_list(value) and length(value) == count, do: value

  def pad_left(value, padder, count) when is_list(value) do
    pad_left([padder | value], padder, count)
  end
end

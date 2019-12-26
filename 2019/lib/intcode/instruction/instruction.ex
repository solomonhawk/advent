defmodule Intcode.Instruction do
  defstruct op: nil, type: nil, parameters: []

  import Intcode.Constants, only: [instruction_length: 1]
  import Intcode.OpCodes, only: [op_type: 1]

  alias Intcode.Instruction.Parameter

  def new(op: op, type: type, parameters: parameters) do
    [instruction, modes] = parse_modes(op)

    %__MODULE__{
      op: instruction,
      type: type,
      parameters: to_params(parameters, modes)
    }
  end

  def instruction_at(program, program_counter) do
    program = Enum.drop(program, program_counter)
    op = instruction_op(hd(program))
    command = Enum.take(program, instruction_length(op))

    new(op: hd(program), type: op_type(op), parameters: tl(command))
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

  def pad_left(value, _, count) when is_list(value) and length(value) == count,
    do: value |> Enum.reverse()

  def pad_left(value, padder, count) when is_list(value) do
    pad_left([padder | value], padder, count)
  end
end

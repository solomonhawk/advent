defmodule Intcode.Fixer do
  require Intcode.OpCodes

  alias Intcode.Instruction
  alias Intcode.Instruction.Parameter
  alias Intcode.OpCodes
  alias Intcode.Constants
  alias Intcode.ExecutionContext

  import Intcode.Instruction.Parameter, only: [deref: 2, value_of: 1]
  import Intcode.Constants, only: [instruction_length: 1]
  import Intcode.Instruction, only: [instruction_at: 2]

  def fix(program) when is_list(program) do
    fix(ExecutionContext.new(program, 0))
  end

  def fix(%ExecutionContext{program: program, instruction_pointer: instruction_pointer} = context) do
    program
    |> instruction_at(instruction_pointer)
    |> execute_instruction(context)
  end

  def execute_instruction(
        %Instruction{op: op, parameters: [p1, p2, p3]},
        %ExecutionContext{program: program, instruction_pointer: instruction_pointer}
      )
      when op == OpCodes.add() do
    program
    |> List.replace_at(value_of(p3), deref(program, p1) + deref(program, p2))
    |> ExecutionContext.new(instruction_pointer + instruction_length(OpCodes.add()))
    |> fix()
  end

  # multiply
  def execute_instruction(
        %Instruction{op: op, parameters: [p1, p2, p3]},
        %ExecutionContext{program: program, instruction_pointer: instruction_pointer}
      )
      when op == OpCodes.mult() do
    program
    |> List.replace_at(value_of(p3), deref(program, p1) * deref(program, p2))
    |> ExecutionContext.new(instruction_pointer + instruction_length(OpCodes.mult()))
    |> fix()
  end

  # input
  def execute_instruction(
        %Instruction{op: op, parameters: [p1]} = instruction,
        %ExecutionContext{} = context
      )
      when op == OpCodes.input() do


    case IO.gets("Enter a value.\n") do
      :eof ->
        raise ArgumentError, message: "Error: input instruction encountered EOF"

      {:error, reason} ->
        raise ArgumentError, message: "Error: input instruction - " <> reason

      data ->
        case parse_input(data) do
          :error ->
            nil

          number ->
            p2 = Parameter.new(value: number, mode: Constants.immediate())

            execute_instruction(
              %Instruction{instruction | parameters: [p1, p2]},
              context
            )
        end
    end
  end

  def execute_instruction(
        %Instruction{op: op, parameters: [p1, p2]},
        %ExecutionContext{program: program, instruction_pointer: instruction_pointer}
      )
      when op == OpCodes.input() do
    program
    |> List.replace_at(value_of(p1), deref(program, p2))
    |> ExecutionContext.new(instruction_pointer + instruction_length(OpCodes.input()))
    |> fix()
  end

  # output
  def execute_instruction(%Instruction{op: op, parameters: [p1]}, %ExecutionContext{
        program: program,
        instruction_pointer: instruction_pointer
      })
      when op == OpCodes.output() do
    IO.write(deref(program, p1))

    program
    |> ExecutionContext.new(instruction_pointer + instruction_length(OpCodes.output()))
    |> fix()
  end

  # halt
  def execute_instruction(%Instruction{op: op}, %ExecutionContext{program: program})
      when op == OpCodes.halt() do
    program
  end

  def parse_input(data) do
    case Integer.parse(data) do
      {number, _} -> number
      :error -> raise ArgumentError, message: "Error: input instruction must be a number"
    end
  end
end

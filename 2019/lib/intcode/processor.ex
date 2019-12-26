defmodule Intcode.Processor do
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
    ExecutionContext.new(program: program) |> fix()
  end

  def fix(%ExecutionContext{program: program, instruction_pointer: instruction_pointer} = context) do
    program
    |> instruction_at(instruction_pointer)
    |> execute_instruction(context)
  end

  # add
  def execute_instruction(
        %Instruction{op: op, parameters: [p1, p2, p3]},
        %ExecutionContext{program: program, instruction_pointer: instruction_pointer} = context
      )
      when op == OpCodes.add() do
    # IO.inspect({:add, {[pointer: p1.value, value: deref(program, p1)]}, {[pointer: p2.value, value: deref(program, p2)]}, {:result, deref(program, p1) + deref(program, p2)}, {:to, p3.value}})
    context
    |> ExecutionContext.update(
      program: List.replace_at(program, value_of(p3), deref(program, p1) + deref(program, p2)),
      instruction_pointer: instruction_pointer + instruction_length(OpCodes.add())
    )
    |> fix()
  end

  # multiply
  def execute_instruction(
        %Instruction{op: op, parameters: [p1, p2, p3]},
        %ExecutionContext{program: program, instruction_pointer: instruction_pointer} = context
      )
      when op == OpCodes.mult() do
    # IO.inspect({:mult, {[pointer: p1.value, value: deref(program, p1)]}, {[pointer: p2.value, value: deref(program, p2)]}, {:result, deref(program, p1) * deref(program, p2)}, {:to, p3.value}})
    context
    |> ExecutionContext.update(
      program: List.replace_at(program, value_of(p3), deref(program, p1) * deref(program, p2)),
      instruction_pointer: instruction_pointer + instruction_length(OpCodes.mult())
    )
    |> fix()
  end

  # input
  def execute_instruction(
        %Instruction{op: op, parameters: [p1]} = instruction,
        %ExecutionContext{} = context
      )
      when op == OpCodes.input() do
    with {number, context} <- ExecutionContext.read(context) do
      p2 = Parameter.new(value: number, mode: Constants.immediate())

      execute_instruction(
        %Instruction{instruction | parameters: [p1, p2]},
        context
      )
    end
  end

  def execute_instruction(
        %Instruction{op: op, parameters: [p1, p2]},
        %ExecutionContext{program: program, instruction_pointer: instruction_pointer} = context
      )
      when op == OpCodes.input() do
    # IO.inspect({:input, {[value: deref(program, p2)]}, {:to, p1.value}})
    context
    |> ExecutionContext.update(
      program: List.replace_at(program, value_of(p1), deref(program, p2)),
      instruction_pointer: instruction_pointer + instruction_length(OpCodes.input())
    )
    |> fix()
  end

  # output
  def execute_instruction(
        %Instruction{op: op, parameters: [p1]},
        %ExecutionContext{
          program: program,
          instruction_pointer: instruction_pointer
        } = context
      )
      when op == OpCodes.output() do
    {_, context} = ExecutionContext.write(context, deref(program, p1))

    # IO.inspect({:output, {:value, {p1.value, deref(program, p1)}}})
    context
    |> ExecutionContext.update(
      program: program,
      instruction_pointer: instruction_pointer + instruction_length(OpCodes.output())
    )
    |> fix()
  end

  # halt
  def execute_instruction(%Instruction{op: op}, %ExecutionContext{} = context)
      when op == OpCodes.halt() do
    context
  end
end

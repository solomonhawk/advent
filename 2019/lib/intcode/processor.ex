defmodule Intcode.Processor do
  require Intcode.OpCodes

  alias Intcode.Instruction
  alias Intcode.Instruction.Parameter
  alias Intcode.OpCodes
  alias Intcode.Constants
  alias Intcode.ExecutionContext

  import Intcode.Instruction.Parameter, only: [deref: 2, value_of: 1]
  import Intcode.Constants, only: [inst_len: 1]
  import Intcode.Instruction, only: [inst_at: 2]

  def fix(program) when is_list(program) do
    ExecutionContext.new(program: program) |> fix()
  end

  def fix(%ExecutionContext{program: program, program_counter: program_counter} = context) do
    program
    |> inst_at(program_counter)
    |> execute_instruction(context)
  end

  # add (1)
  def execute_instruction(
        %Instruction{op: op, parameters: [p1, p2, p3]},
        %ExecutionContext{program: program, program_counter: program_counter} = context
      )
      when op == OpCodes.add() do
    context
    |> struct(
      program: List.replace_at(program, value_of(p3), deref(program, p1) + deref(program, p2)),
      program_counter: program_counter + inst_len(OpCodes.add())
    )
    |> fix()
  end

  # multiply (2)
  def execute_instruction(
        %Instruction{op: op, parameters: [p1, p2, p3]},
        %ExecutionContext{program: program, program_counter: program_counter} = context
      )
      when op == OpCodes.mult() do
    context
    |> struct(
      program: List.replace_at(program, value_of(p3), deref(program, p1) * deref(program, p2)),
      program_counter: program_counter + inst_len(OpCodes.mult())
    )
    |> fix()
  end

  # input (3)
  def execute_instruction(
        %Instruction{op: op, parameters: [p1]} = instruction,
        %ExecutionContext{} = context
      )
      when op == OpCodes.input() do
    with {number, context} <- ExecutionContext.read(context) do
      p2 = Parameter.new(value: number, mode: Constants.immediate())

      execute_instruction(
        struct(instruction, parameters: [p1, p2]),
        context
      )
    end
  end

  def execute_instruction(
        %Instruction{op: op, parameters: [p1, p2]},
        %ExecutionContext{program: program, program_counter: program_counter} = context
      )
      when op == OpCodes.input() do
    context
    |> struct(
      program: List.replace_at(program, value_of(p1), deref(program, p2)),
      program_counter: program_counter + inst_len(OpCodes.input())
    )
    |> fix()
  end

  # output (4)
  def execute_instruction(
        %Instruction{op: op, parameters: [p1]},
        %ExecutionContext{
          program: program,
          program_counter: program_counter
        } = context
      )
      when op == OpCodes.output() do
    {_, context} = ExecutionContext.write(context, deref(program, p1))

    context
    |> struct(program_counter: program_counter + inst_len(OpCodes.output()))
    |> fix()
  end

  # jump_if_true (5)
  def execute_instruction(
        %Instruction{op: op, parameters: [p1, p2]},
        %ExecutionContext{program: program, program_counter: program_counter} = context
      )
      when op == OpCodes.jump_if_true() do
    should_jump = deref(program, p1) != 0
    destination = deref(program, p2)

    context
    |> jump(should_jump, program_counter, OpCodes.jump_if_true(), destination)
    |> fix()
  end

  # jump_if_false (6)
  def execute_instruction(
        %Instruction{op: op, parameters: [p1, p2]},
        %ExecutionContext{program: program, program_counter: program_counter} = context
      )
      when op == OpCodes.jump_if_false() do
    should_jump = deref(program, p1) == 0
    destination = deref(program, p2)

    context
    |> jump(should_jump, program_counter, OpCodes.jump_if_false(), destination)
    |> fix()
  end

  # less_than (7)
  def execute_instruction(
        %Instruction{op: op, parameters: [p1, p2, p3]},
        %ExecutionContext{program: program, program_counter: program_counter} = context
      )
      when op == OpCodes.less_than() do
    value = if(deref(program, p1) < deref(program, p2), do: 1, else: 0)

    context
    |> struct(
      program: List.replace_at(program, value_of(p3), value),
      program_counter: program_counter + inst_len(OpCodes.less_than())
    )
    |> fix()
  end

  # equals (8)
  def execute_instruction(
        %Instruction{op: op, parameters: [p1, p2, p3]},
        %ExecutionContext{program: program, program_counter: program_counter} = context
      )
      when op == OpCodes.equals() do
    value = if(deref(program, p1) == deref(program, p2), do: 1, else: 0)

    context
    |> struct(
      program: List.replace_at(program, value_of(p3), value),
      program_counter: program_counter + inst_len(OpCodes.equals())
    )
    |> fix()
  end

  # halt
  def execute_instruction(%Instruction{op: op}, %ExecutionContext{} = context)
      when op == OpCodes.halt() do
    context
  end

  def jump(context, false, program_counter, instruction, _) do
    struct(context, program_counter: program_counter + inst_len(instruction))
  end

  def jump(context, true, _, _, destination) do
    struct(context, program_counter: destination)
  end
end

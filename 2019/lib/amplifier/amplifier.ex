defmodule Amplifier do
  alias Intcode.{Processor, ExecutionContext}
  alias Intcode.ExecutionContext.Adapters.Memory, as: MemoryAdapter

  @inputs [0, 1, 2, 3, 4]

  def optimize(program) do
    Enum.reduce(permutations(@inputs), 0, fn phase_settings, max_thrust ->
      max(max_thrust, Amplifier.calculate(program, phase_settings))
    end)
  end

  def calculate(program, phase_settings) do
    Enum.reduce(phase_settings, 0, fn phase_setting, previous_output ->
      program
      |> context([phase_setting, previous_output])
      |> Processor.run()
      |> ExecutionContext.outputs()
      |> List.last()
    end)
  end

  def context(program, inputs) do
    ExecutionContext.new(program: program, adapter: MemoryAdapter.new(inputs: inputs))
  end

  def permutations([]), do: [[]]
  def permutations(list) do
    for elem <- list, rest <- permutations(list -- [elem]), do: [elem|rest]
  end
end

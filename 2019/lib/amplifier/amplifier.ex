defmodule Amplifier do
  alias Intcode.{Processor, ExecutionContext}
  alias Intcode.ExecutionContext.Adapters.Memory, as: MemoryAdapter

  import Helpers.List, only: [permutations: 1]

  @series_phases [0, 1, 2, 3, 4]
  @loop_phases [5, 6, 7, 8, 9]

  def optimize(program, mode \\ :series) do
    mode
    |> inputs_for_mode()
    |> permutations()
    |> Enum.reduce(0, fn phase_settings, max_thrust ->
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

  def inputs_for_mode(:series), do: @series_phases
  def inputs_for_mode(:loop), do: @loop_phases
  def inputs_for_mode(mode), do: raise("Invalid amplifier mode '#{mode}'")

  def context(program, inputs) do
    ExecutionContext.new(program: program, adapter: MemoryAdapter.new(inputs: inputs))
  end
end

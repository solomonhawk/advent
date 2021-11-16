defmodule Amplifier.Worker do
  use GenServer, restart: :transient

  alias Intcode.ExecutionContext
  alias Intcode.ExecutionContext.Adapters.Memory, as: MemoryAdapter

  def start_link(args) do
    id = Keyword.get(args, :id)
    name = Keyword.get(args, :name)

    GenServer.start_link(
      __MODULE__,
      args,
      name: {:global, name <> to_string(id)}
    )
  end

  @impl true
  def init(
        id: id,
        name: name,
        program: program,
        phase_setting: phase_setting,
        source: source,
        dest: dest
      ) do
    state = %{
      id: id,
      name: name,
      source: name <> to_string(source),
      dest: name <> to_string(dest),
      phase_setting: phase_setting,
      result: nil,
      context:
        ExecutionContext.new(
          program: program,
          adapter: MemoryAdapter.new(),
          opts: [throw_errors: false]
        )
    }

    {:ok, state, {:continue, :initialize}}
  end

  @impl true
  def handle_continue(:initialize, %{phase_setting: phase_setting} = state) do
    {:noreply, on_input(state, phase_setting)}
  end

  @impl true
  def handle_cast({:input, value}, state) do
    case on_input(state, value) do
      {:stop, :normal, new_state} ->
        {:stop, :normal, new_state}

      new_state ->
        {:noreply, new_state}
    end
  end

  def on_input(%{id: id, name: name, dest: dest, context: context} = state, input) do
    {output, next_context} =
      context
      |> ExecutionContext.put_input(input)
      |> Intcode.Processor.run()
      |> ExecutionContext.take_output()

    if output do
      GenServer.cast({:global, dest}, {:input, output})
    end

    if ExecutionContext.halted?(next_context) do
      {:stop, :normal, %{state | context: next_context, result: output}}
    else
      %{state | context: next_context, result: output}
    end
  end

  @impl true
  def terminate(:normal, %{id: id, name: name, result: result}) do
    if id == :E do
      Amplifier.Aggregator.add_result(result)
    end
  end

  @impl true
  def terminate(_, _) do
    nil
  end
end

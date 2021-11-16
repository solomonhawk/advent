defmodule Amplifier.DynamicSupervisor do
  use DynamicSupervisor

  @labels [:A, :B, :C, :D, :E]

  def start_link(init_arg, name \\ __MODULE__) do
    DynamicSupervisor.start_link(__MODULE__, init_arg, name: name)
  end

  def start_child(name, child_spec) do
    DynamicSupervisor.start_child(name, child_spec)
  end

  def optimize({:global, name}, program, %Amplifier.Configuration{
        phase_settings: phase_settings
      }) do
    count = Enum.count(phase_settings)

    Range.new(0, count - 1)
    |> Enum.map(fn id ->
      start_child({:global, name}, {
        Amplifier.Worker,
        id: Enum.at(@labels, id),
        name: name,
        program: program,
        phase_setting: Enum.at(phase_settings, id),
        source: worker_id(count, id - 1),
        dest: worker_id(count, id + 1)
      })
    end)

    # start the amplifier optimization sequence
    GenServer.cast({:global, name <> to_string(:A)}, {:input, 0})
  end

  def worker_id(range, index) do
    Enum.at(@labels, rem(rem(index, range) + range, range))
  end

  @impl true
  def init(_init_arg) do
    DynamicSupervisor.init(strategy: :one_for_one)
  end
end

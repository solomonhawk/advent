defmodule Intcode.ExecutionContext do
  alias Intcode.ExecutionContext.Adapters

  defstruct program: [],
            program_counter: 0,
            adapter: %Adapters.IO{},
            opts: [throw_errors: true],
            events: []

  def new(attrs) do
    struct(__MODULE__, attrs)
  end

  def program(%__MODULE__{program: program}), do: program

  def outputs(%__MODULE__{adapter: adapter}), do: Adapter.outputs(adapter)

  def events(%__MODULE__{events: events}), do: Enum.reverse(events)

  def program_counter(%__MODULE__{program_counter: program_counter}),
    do: program_counter

  def read(%__MODULE__{adapter: adapter} = context) do
    case Adapter.request_input(adapter) do
      {:error, reason} ->
        handle_error(context, reason)

      {:ok, result, updated_adapter} ->
        {result, struct(context, adapter: updated_adapter) |> put_event(:read, result)}
    end
  end

  def write(%__MODULE__{adapter: adapter} = context, output) do
    case Adapter.request_output(adapter, output) do
      {:error, reason} ->
        handle_error(context, reason)

      {:ok, result, updated_adapter} ->
        {result, struct(context, adapter: updated_adapter) |> put_event(:write, result)}
    end
  end

  def put_event(%__MODULE__{events: events} = context, event, args) do
    struct(context, events: [{event, args} | events])
  end

  def put_error(%__MODULE__{adapter: adapter} = context, error) do
    struct(context, adapter: struct(adapter, error: error))
  end

  def put_input(%__MODULE__{adapter: adapter} = context, input) do
    struct(context, adapter: Adapter.put_input(adapter, input))
  end

  def handle_error(context, reason) do
    if Keyword.get(context.opts, :throw_errors, true) == true do
      raise RuntimeError, message: reason
    end

    put_error(context, reason)
  end
end

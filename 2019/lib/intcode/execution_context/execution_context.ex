defmodule Intcode.ExecutionContext do
  alias Intcode.ExecutionContext.Adapters

  defstruct program: [],
            program_counter: 0,
            status: :idle,
            adapter: %Adapters.IO{},
            opts: [throw_errors: true],
            events: []

  @type status ::
          :idle
          | :waiting
          | :error
          | :halted

  @type t :: %__MODULE__{
          program: list(integer()),
          program_counter: non_neg_integer(),
          status: status(),
          adapter: struct(),
          opts: [throw_errors: boolean()],
          events: list(tuple())
        }

  def new(attrs) do
    struct(__MODULE__, attrs)
  end

  def halted?(%__MODULE__{status: :halted}), do: true
  def halted?(%__MODULE__{}), do: false

  def status(%__MODULE__{status: status}), do: status

  def program(%__MODULE__{program: program}), do: program

  def outputs(%__MODULE__{adapter: adapter}), do: Adapter.outputs(adapter)

  def events(%__MODULE__{events: events}), do: Enum.reverse(events)

  def program_counter(%__MODULE__{program_counter: program_counter}),
    do: program_counter

  def read(%__MODULE__{adapter: adapter} = context) do
    case Adapter.request_input(adapter) do
      {:error, status, reason} ->
        handle_error(context, status, reason)

      {:ok, result, updated_adapter} ->
        {result, struct(context, adapter: updated_adapter) |> put_event(:read, result)}
    end
  end

  def write(%__MODULE__{adapter: adapter} = context, output) do
    case Adapter.request_output(adapter, output) do
      {:error, status, reason} ->
        handle_error(context, status, reason)

      {:ok, result, updated_adapter} ->
        {result, struct(context, adapter: updated_adapter) |> put_event(:write, result)}
    end
  end

  def put_event(%__MODULE__{events: events} = context, event, args) do
    struct(context, events: [{event, args} | events])
  end

  def put_error(%__MODULE__{adapter: adapter} = context, error) do
    struct(context, status: :error, adapter: struct(adapter, error: error))
  end

  def put_status(%__MODULE__{} = context, status) do
    struct(context, status: status)
  end

  def put_input(%__MODULE__{adapter: adapter} = context, input) do
    struct(context, adapter: Adapter.put_input(adapter, input))
  end

  def take_output(%__MODULE__{adapter: adapter} = context) do
    {output, adapter} = Adapter.take_output(adapter)
    {output, struct(context, adapter: adapter)}
  end

  def handle_error(context, status, reason) do
    if Keyword.get(context.opts, :throw_errors, true) == true do
      raise RuntimeError, message: reason
    end

    case status do
      :no_input_available ->
        put_status(context, :waiting)

      _ ->
        put_error(context, reason)
    end
  end
end

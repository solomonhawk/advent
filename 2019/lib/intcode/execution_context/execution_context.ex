defmodule Intcode.ExecutionContext do
  alias Intcode.ExecutionContext.Adapters

  defstruct program: [],
            instruction_pointer: 0,
            adapter: %Adapters.IO{},
            events: []

  def new(attrs) do
    struct(__MODULE__, attrs)
  end

  def update(context, attrs) do
    struct(__MODULE__, Keyword.merge(Map.to_list(context), attrs))
  end

  def program(%__MODULE__{program: program}), do: program

  def events(%__MODULE__{events: events}), do: Enum.reverse(events)

  def instruction_pointer(%__MODULE__{instruction_pointer: instruction_pointer}),
    do: instruction_pointer

  def read(%__MODULE__{adapter: adapter} = context) do
    {result, context} = adapter.__struct__.request_input(context)

    case result do
      :error ->
        raise ArgumentError, message: context.adapter.error

      _ ->
        {result, put_event(context, :read, result)}
    end
  end

  def write(%__MODULE__{adapter: adapter} = context, output) do
    {result, context} = adapter.__struct__.request_output(context, output)

    case result do
      :error ->
        raise ArgumentError, message: context.adapter.error

      _ ->
        {result, put_event(context, :write, result)}
    end
  end

  def put_event(%__MODULE__{events: events} = context, event, args) do
    %__MODULE__{context | events: [{event, args} | events]}
  end
end

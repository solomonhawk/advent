defmodule Intcode.ExecutionContext.Adapters.Memory do
  defstruct inputs: [], outputs: [], error: nil

  alias Intcode.ExecutionContext

  def new(attrs) do
    struct(__MODULE__, attrs)
  end

  def outputs(context), do: context.adapter.outputs

  def request_input(
        %ExecutionContext{adapter: %__MODULE__{inputs: [input | inputs]} = adapter} = context
      ) do
    {input, %ExecutionContext{context | adapter: %__MODULE__{adapter | inputs: inputs}}}
  end

  def request_input(%ExecutionContext{adapter: %__MODULE__{inputs: []} = adapter} = context) do
    {:error,
     ExecutionContext.update(context,
       adapter: %__MODULE__{adapter | error: "No inputs left to get."}
     )}
  end

  def request_output(
        %ExecutionContext{adapter: %__MODULE__{outputs: outputs} = adapter} = context,
        output
      ) do
    {output,
     ExecutionContext.update(context,
       adapter: %__MODULE__{adapter | outputs: outputs ++ [output]}
     )}
  end
end

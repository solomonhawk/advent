defmodule Intcode.ExecutionContext.Adapters.Memory do
  defstruct inputs: [], outputs: [], error: nil

  def new(attrs) do
    struct(__MODULE__, attrs)
  end
end

defimpl Adapter, for: Intcode.ExecutionContext.Adapters.Memory do
  alias Intcode.ExecutionContext
  alias Intcode.ExecutionContext.Adapters.Memory, as: MemoryAdapter

  def request_input(%MemoryAdapter{inputs: [input | inputs]} = adapter) do
    {:ok, input, struct(adapter, inputs: inputs)}
  end

  def request_input(%ExecutionContext{adapter: %MemoryAdapter{inputs: []} = adapter} = context) do
    {:error, "No inputs left to get."}
  end

  def request_output(%MemoryAdapter{outputs: outputs} = adapter, output) do
    {:ok, output, struct(adapter, outputs: outputs ++ [output])}
  end

  def outputs(adapter), do: adapter.outputs
end

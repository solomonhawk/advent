defmodule Intcode.ExecutionContext.Adapters.Memory do
  defstruct inputs: [], outputs: [], error: nil

  def new(attrs \\ []) do
    struct(__MODULE__, attrs)
  end
end

defimpl Adapter, for: Intcode.ExecutionContext.Adapters.Memory do
  alias Intcode.ExecutionContext
  alias Intcode.ExecutionContext.Adapters.Memory, as: MemoryAdapter

  def request_input(%MemoryAdapter{inputs: [input | inputs]} = adapter) do
    {:ok, input, struct(adapter, inputs: inputs)}
  end

  def request_input(%ExecutionContext{adapter: %MemoryAdapter{inputs: []}}) do
    {:error, "No inputs left to get."}
  end

  def request_output(%MemoryAdapter{outputs: outputs} = adapter, output) do
    {:ok, output, struct(adapter, outputs: [output | outputs])}
  end

  def put_input(%MemoryAdapter{inputs: inputs} = adapter, input) do
    struct(adapter, inputs: inputs ++ [input])
  end

  def outputs(adapter), do: adapter.outputs |> Enum.reverse()
end

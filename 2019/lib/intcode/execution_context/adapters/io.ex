defmodule Intcode.ExecutionContext.Adapters.IO do
  defstruct error: nil
end

defimpl Adapter, for: Intcode.ExecutionContext.Adapters.IO do
  def request_input(adapter) do
    case IO.gets("Enter a number.\n") do
      :eof ->
        {:error, :bad_input, "Error: input instruction encountered EOF"}

      {:error, reason} ->
        {:error, :bad_input, "Error: input instruction - " <> reason}

      data ->
        case Integer.parse(data) do
          {number, _} ->
            {:ok, number, adapter}

          :error ->
            {:error, :bad_input, "Error: input instruction - must be an integer"}
        end
    end
  end

  def request_output(adapter, output) do
    IO.write(output)
    {:ok, output, adapter}
  end

  def outputs(_), do: []

  def put_input(adapter, _), do: adapter
  def take_output(adapter), do: adapter
end

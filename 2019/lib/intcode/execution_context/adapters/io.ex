defmodule Intcode.ExecutionContext.Adapters.IO do
  defstruct error: nil

  alias Intcode.ExecutionContext

  def request_input(context) do
    case IO.gets("Enter a number.\n") do
      :eof ->
        {:error, put_error(context, "Error: input instruction encountered EOF")}

      {:error, reason} ->
        {:error, put_error(context, "Error: input instruction - " <> reason)}

      data ->
        case Integer.parse(data) do
          {number, _} ->
            {number, context}

          :error ->
            {:error, put_error(context, "Error: input instruction - must be an integer")}
        end
    end
  end

  def request_output(context, output) do
    IO.write(output)

    {output, context}
  end

  def put_error(%ExecutionContext{adapter: adapter} = context, error) do
    ExecutionContext.update(context, adapter: %__MODULE__{adapter | error: error})
  end
end

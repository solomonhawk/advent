defmodule Amplifier.Aggregator do
  use Agent

  def start_link(%{total: total, reply_to: reply_to}) do
    Agent.start_link(fn -> initial_state(total, reply_to) end,
      name: __MODULE__
    )
  end

  def add_result(result) do
    Agent.update(__MODULE__, &handle_result(result, &1))
  end

  defp initial_state(total, reply_to) do
    %{total: total, count: 0, best_result: 0, reply_to: reply_to}
  end

  defp handle_result(
         result,
         %{total: total, count: count, best_result: best_result, reply_to: reply_to} = state
       )
       when is_pid(reply_to) do
    if count + 1 == total do
      send(reply_to, {:done, max(best_result, result)})
      Agent.stop(__MODULE__)
    end

    %{state | count: count + 1, best_result: max(best_result, result)}
  end
end

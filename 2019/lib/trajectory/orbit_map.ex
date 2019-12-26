defmodule Trajectory.OrbitMap do
  defstruct input: [], graph: %{}

  alias Trajectory.Graph

  def new(input) when is_list(input) do
    struct(__MODULE__, input: input)
    |> parse_graph()
  end

  def count_orbits(%__MODULE__{graph: graph}) do
    count_orbits(graph, Graph.ids(graph), 0)
  end

  def count_orbits(graph, [id | ids], orbits) do
    count_orbits(graph, ids, orbits + Graph.parent_count(graph, id))
  end

  def count_orbits(_, [], orbits) do
    orbits
  end

  # private

  defp parse_graph(%__MODULE__{graph: graph, input: [{t, s} | rest]} = map) do
    parse_graph(struct(map, input: rest, graph: Map.update(graph, t, [s], &[s | &1])))
  end

  defp parse_graph(%__MODULE__{graph: graph, input: []} = map) do
    struct(map, graph: Graph.new(graph))
  end

end

defmodule Trajectory.Graph do
  defstruct nodes: []

  alias Trajectory.Node

  def new(map) do
    struct(__MODULE__, nodes: nodes_from_map(map))
  end

  def ids(graph) do
    Map.keys(graph.nodes)
  end

  def node(graph, name) do
    Map.get(graph.nodes, name)
  end

  def parent_count(graph, node_name) do
    parents(graph, node_name) |> length()
  end

  def parents(graph, node_name) when is_binary(node_name) or is_atom(node_name) do
    case Map.get(graph.nodes, node_name) do
      nil -> []
      node -> parents(graph, node, [])
    end
  end

  def parents(graph, %Node{parent: parent_name}, parent_ids) when parent_name != nil do
    [parent_name | parent_ids] ++ parents(graph, parent_name)
  end

  def parents(_, %Node{root: true}, parents) do
    parents
  end

  def min_distance(graph, %Node{name: name_from}, %Node{name: name_to}) do
    min_distance(graph, name_from, name_to)
  end

  def min_distance(graph, from, to) do
    nearest_ancestor =
      case ancestors(graph, from, to) do
        :error -> :error
        list -> List.last(list)
      end

    distance(graph, from, nearest_ancestor) + distance(graph, to, nearest_ancestor)
  end

  def ancestors(graph, from, to) do
    parents_from = parents(graph, from)
    parents_to = parents(graph, to)

    common_ancestors = list_intersection(parents_from, parents_to)

    case common_ancestors |> length() > 0 do
      true -> common_ancestors
      false -> :error
    end
  end

  # private

  defp nodes_from_map(map) do
    Enum.reduce(map, %{}, fn {target, satellites}, nodes ->
      nodes
      |> add_node(%Node{name: target, children: MapSet.new(satellites)})
      |> set_parent(satellites, target)
    end)
    |> find_root_and_leaf_nodes()
  end

  defp add_node(nodes, node) do
    Map.update(nodes, node.name, node, fn existing ->
      struct(existing, %{
        parent: existing.parent || node.parent,
        children: MapSet.union(existing.children, node.children),
        root: existing.root || node.root
      })
    end)
  end

  defp set_parent(nodes, satellites, parent) do
    Enum.reduce(satellites, nodes, fn name, nodes ->
      add_node(nodes, %Node{name: name, parent: parent})
    end)
  end

  defp find_root_and_leaf_nodes(nodes) do
    Enum.reduce(nodes, nodes, fn {name, node}, nodes ->
      Map.update(nodes, name, node, fn node ->
        struct(node, root: node.parent == nil, leaf: MapSet.size(node.children) == 0)
      end)
    end)
  end

  defp distance(graph, from, to)
       when (is_binary(from) or is_atom(from)) and (is_binary(to) or is_atom(to)) do
    distance(graph, node(graph, from), node(graph, to), 0)
  end

  defp distance(_, %Node{name: name}, %Node{name: name}, d) do
    d
  end

  defp distance(graph, %Node{parent: parent}, %Node{} = to, d) do
    distance(graph, node(graph, parent), to, d + 1)
  end

  defp list_intersection(list_a, list_b) do
    {list_a, list_b} = order_by_size(list_a, list_b)

    Enum.reduce(list_a, [], fn a, acc ->
      case Enum.find(list_b, fn b -> a == b end) do
        nil -> acc
        element -> [element | acc]
      end
    end)
  end

  defp order_by_size(list1, list2) when length(list1) > length(list2), do: {list2, list1}
  defp order_by_size(list1, list2), do: {list1, list2}
end

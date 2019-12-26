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

  def parent_count(graph, node_name) when is_binary(node_name) or is_atom(node_name) do
    case Map.get(graph.nodes, node_name) do
      nil -> 0
      node -> parent_count(graph, node, 0)
    end
  end

  def parent_count(graph, %Node{parent: parent_name, root: false}, count) do
    count + 1 + parent_count(graph, parent_name)
  end

  def parent_count(_, %Node{root: true}, count) do
    count
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
end

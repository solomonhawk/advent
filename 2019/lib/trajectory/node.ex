defmodule Trajectory.Node do
  defstruct name: nil,
            parent: nil,
            children: MapSet.new(),
            root: false,
            leaf: false
end

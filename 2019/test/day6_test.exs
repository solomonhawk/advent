defmodule Day6Test do
  use ExUnit.Case

  alias Trajectory.{OrbitMap, Graph}

  @tag :six
  describe "Trajectory.OrbitMap" do
    test "new/1 parses the input map into a graph when created" do
      subject = OrbitMap.new([{"A", "B"}, {"B", "C"}, {"B", "D"}])

      assert %Graph{} = subject.graph
    end

    test "count_orbits/1 counts the total direct and indirect orbits" do
      subject = OrbitMap.new([{"A", "B"}, {"B", "C"}, {"B", "D"}])

      assert OrbitMap.count_orbits(subject) == 5
    end
  end

  describe "part 1" do
    test "computes the checksum correctly" do
      assert Day6.Part1.run() == 140608
    end
  end
end

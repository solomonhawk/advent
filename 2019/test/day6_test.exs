defmodule Day6Test do
  use ExUnit.Case

  alias Trajectory.{OrbitMap, Graph}

  @map """
       COM)B
       B)C
       C)D
       D)E
       E)F
       B)G
       G)H
       D)I
       E)J
       J)K
       K)L
       K)YOU
       I)SAN
       """
       |> Trajectory.parse()
       |> OrbitMap.new()

  describe "Trajectory.OrbitMap" do
    test "new/1 parses the input map into a graph when created" do
      subject = OrbitMap.new([{"A", "B"}, {"B", "C"}, {"B", "D"}])

      assert %Graph{} = subject.graph
    end

    test "count_orbits/1 counts the total direct and indirect orbits" do
      subject = OrbitMap.new([{"A", "B"}, {"B", "C"}, {"B", "D"}])

      assert OrbitMap.count_orbits(subject) == 5
    end

    test "parents/2 of YOU" do
      assert Graph.parents(@map.graph, "YOU") == ["K", "J", "E", "D", "C", "B", :com]
    end

    test "parents/2 of SAN" do
      assert Graph.parents(@map.graph, "SAN") == ["I", "D", "C", "B", :com]
    end

    test "minimum orbital transfer distance" do
      assert OrbitMap.orbital_transfers(@map, "YOU", "SAN") == 4
    end
  end

  describe "part 1" do
    test "computes the checksum correctly" do
      assert Day6.Part1.run() == 140_608
    end
  end

  describe "part 2" do
    test "computes the minimum orbital transfers correctly" do
      assert Day6.Part2.run() == 337
    end
  end
end

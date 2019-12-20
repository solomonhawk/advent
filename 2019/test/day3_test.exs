defmodule Day3Test do
  use ExUnit.Case

  describe "Wire.Parser" do
    test "parse_wires/1" do
      assert Wire.Parser.parse_wires("R1,D2\nL3,U4") == {[{:right, 1}, {:down, 2}], [{:left, 3}, {:up, 4}]}
    end
  end

  describe "Wire.Fixer" do
    test "distance/2 calculates the Manhattan distance between two points" do
      assert Wire.Fixer.distance({0, 0}, {5, 5}) == 10
    end

    test "uncross/1 example 1" do
      assert Wire.Fixer.uncross({[right: 8, up: 5, left: 5, down: 3], [up: 7, right: 6, down: 4, left: 4]}) == 6
    end

    test "uncross/1 example 2" do
      assert Wire.Fixer.uncross({
        [{:right, 75}, {:down, 30}, {:right, 83}, {:up, 83}, {:left, 12}, {:down, 49}, {:right, 71}, {:up, 7}, {:left, 72}],
        [{:up, 62}, {:right, 66}, {:up, 55}, {:right, 34}, {:down, 71}, {:right, 55}, {:down, 58}, {:right, 83}]
      }) == 159
    end

    test "uncross/1 example 3" do
      assert Wire.Fixer.uncross({
        [{:right, 98}, {:up, 47}, {:right, 26}, {:down, 63}, {:right, 33}, {:up, 87}, {:left, 62}, {:down, 20}, {:right, 33}, {:up, 53}, {:right, 51}],
        [{:up, 98}, {:right, 91}, {:down, 20}, {:right, 16}, {:down, 67}, {:right, 40}, {:up, 7}, {:right, 15}, {:up, 6}, {:right, 7}]
      }) == 135
    end
  end

  describe "part 1" do
    # test "the prepared program, after being fixed, yields the correct instruction at position 0" do
    #   assert Day2.Part1.run == 6327510
    # end
  end

  describe "part 2" do
    # test "run" do
    #   # the final result is (100 * number1 + number2),
    #   # where number1 is substituted at position 1
    #   # and number2 is substituted at position 2
    #   assert Day2.Part2.run == 4112
    # end
  end
end

defmodule Day2Test do
  use ExUnit.Case

  alias Intcode.ExecutionContext

  describe "Intcode.Processor" do
    test "1,0,0,0,99 becomes 2,0,0,0,99 (1 + 1 = 2)" do
      assert Intcode.Processor.run([1, 0, 0, 0, 99]) |> ExecutionContext.program() == [
               2,
               0,
               0,
               0,
               99
             ]
    end

    test "2,3,0,3,99 becomes 2,3,0,6,99 (3 * 2 = 6)" do
      assert Intcode.Processor.run([2, 3, 0, 3, 99]) |> ExecutionContext.program() == [
               2,
               3,
               0,
               6,
               99
             ]
    end

    test "2,4,4,5,99,0 becomes 2,4,4,5,99,9801 (99 * 99 = 9801)" do
      assert Intcode.Processor.run([2, 4, 4, 5, 99, 0]) |> ExecutionContext.program() == [
               2,
               4,
               4,
               5,
               99,
               9801
             ]
    end

    test "1,1,1,4,99,5,6,0,99 becomes 30,1,1,4,2,5,6,0,99" do
      assert Intcode.Processor.run([1, 1, 1, 4, 99, 5, 6, 0, 99]) |> ExecutionContext.program() ==
               [30, 1, 1, 4, 2, 5, 6, 0, 99]
    end
  end

  describe "part 1" do
    test "the prepared program, after being fixed, yields the correct instruction at position 0" do
      assert Day2.Part1.run() == 6_327_510
    end
  end

  describe "part 2" do
    test "the prepared program yields 19690720 in position 0 when the correct values are substituted for position 1 and 2" do
      # the final result is (100 * number1 + number2),
      # where number1 is substituted at position 1
      # and number2 is substituted at position 2
      assert Day2.Part2.run() == 4112
    end
  end
end

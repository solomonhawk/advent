defmodule Day7Test do
  use ExUnit.Case

  describe "Amplifier" do
    test "is correct for the first sample" do
      program = [3,15,3,16,1002,16,10,16,1,16,15,15,4,15,99,0,0]
      assert Amplifier.optimize(program) == 43210
    end

    test "is correct for the second sample" do
      program = [3,23,3,24,1002,24,10,24,1002,23,-1,23,101,5,23,23,1,24,23,23,4,23,99,0,0]
      assert Amplifier.optimize(program) == 54321
    end

    test "is correct for the third example" do
      program = [3,31,3,32,1002,32,10,32,1001,31,-2,31,1007,31,0,33,1002,33,7,33,1,33,31,31,1,32,31,31,4,31,99,0,0,0]
      assert Amplifier.optimize(program) == 65210
    end
  end

  describe "part 1" do
    test "identifies the correct optimal amplifier thrust value" do
      assert Day7.Part1.run() == 14_902
    end
  end

  describe "part 2" do
    test "is correct" do

    end
  end
end

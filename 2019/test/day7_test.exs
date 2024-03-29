defmodule Day7Test do
  use ExUnit.Case

  describe "Amplifier" do
    test "is correct for the first sample" do
      program = [3, 15, 3, 16, 1002, 16, 10, 16, 1, 16, 15, 15, 4, 15, 99, 0, 0]
      assert Amplifier.optimize(program) == 43210
    end

    test "is correct for the second sample" do
      program = [
        3,
        23,
        3,
        24,
        1002,
        24,
        10,
        24,
        1002,
        23,
        -1,
        23,
        101,
        5,
        23,
        23,
        1,
        24,
        23,
        23,
        4,
        23,
        99,
        0,
        0
      ]

      assert Amplifier.optimize(program) == 54321
    end

    test "is correct for the third example" do
      program = [
        3,
        31,
        3,
        32,
        1002,
        32,
        10,
        32,
        1001,
        31,
        -2,
        31,
        1007,
        31,
        0,
        33,
        1002,
        33,
        7,
        33,
        1,
        33,
        31,
        31,
        1,
        32,
        31,
        31,
        4,
        31,
        99,
        0,
        0,
        0
      ]

      assert Amplifier.optimize(program) == 65210
    end
  end

  describe "part 1" do
    test "identifies the correct optimal amplifier thrust value" do
      assert Day7.Part1.run() == 14_902
    end
  end

  describe "part 2" do
    test "sample program 1 with phase settings 9,8,7,6,5 returns max thrust of 139629729" do
      program = [
        3,
        26,
        1001,
        26,
        -4,
        26,
        3,
        27,
        1002,
        27,
        2,
        27,
        1,
        27,
        26,
        27,
        4,
        27,
        1001,
        28,
        -1,
        28,
        1005,
        28,
        6,
        99,
        0,
        0,
        5
      ]

      assert Day7.Part2.calculate(program) == 139_629_729
    end

    test "sample program 2 with phase settings 9,7,8,5,6 returns max thrust of 18216" do
      program = [
        3,
        52,
        1001,
        52,
        -5,
        52,
        3,
        53,
        1,
        52,
        56,
        54,
        1007,
        54,
        5,
        55,
        1005,
        55,
        26,
        1001,
        54,
        -5,
        54,
        1105,
        1,
        12,
        1,
        53,
        54,
        53,
        1008,
        54,
        0,
        55,
        1001,
        55,
        1,
        55,
        2,
        53,
        55,
        53,
        4,
        53,
        1001,
        56,
        -1,
        56,
        1005,
        56,
        6,
        99,
        0,
        0,
        0,
        0,
        10
      ]

      assert Day7.Part2.calculate(program) == 18216
    end

    test "identifies the correct optimal amplifier thrust value" do
      assert Day7.Part2.run() == 6_489_132
    end
  end
end

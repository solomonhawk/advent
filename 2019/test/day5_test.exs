defmodule Day5Test do
  use ExUnit.Case

  import ExUnit.CaptureIO

  describe "upgraded Intcode.Fixer" do
    test "3,0,4,0,99 gets an input from :stdin and outputs it to :stdout" do
      output =
        capture_io([input: "888", capture_prompt: false], fn ->
          Intcode.Fixer.fix([3, 0, 4, 0, 99])
        end)

      assert output == "888"
    end
  end

  describe "part 1" do
    test "oof" do
      # assert Day2.Part1.run() == 6_327_510
    end
  end

  describe "part 2" do
    # test "the prepared program yields 19690720 in position 0 when the correct values are substituted for position 1 and 2" do
    #   # the final result is (100 * number1 + number2),
    #   # where number1 is substituted at position 1
    #   # and number2 is substituted at position 2
    #   assert Day2.Part2.run() == 4112
    # end
  end
end

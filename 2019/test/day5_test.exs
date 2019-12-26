defmodule Day5Test do
  use ExUnit.Case

  import ExUnit.CaptureIO

  alias Intcode.ExecutionContext
  alias Intcode.ExecutionContext.Adapters.Memory, as: MemoryAdapter

  describe "Intcode.Processor with IO Adapter" do
    test "3,0,4,0,99 gets an input from :stdin and outputs it to :stdout" do
      output =
        capture_io([input: "888", capture_prompt: false], fn ->
          Intcode.Processor.fix([3, 0, 4, 0, 99])
        end)

      assert output == "888"
    end
  end

  describe "Intcode.Processor with Memory Adapter" do
    test "3,0,4,0,99 gets an input from the adapter and outputs it to the adapter" do
      context =
        ExecutionContext.new(program: [3, 0, 4, 0, 99], adapter: MemoryAdapter.new(inputs: [888]))
        |> Intcode.Processor.fix()

      assert ExecutionContext.events(context) == [read: 888, write: 888]
      assert MemoryAdapter.outputs(context) == [888]
    end
  end

  describe "part 1" do
    test "runs the TEST diagnostic program and gets the correct result" do
      assert Day5.Part1.run() == 7_839_346
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

defmodule Day5Test do
  use ExUnit.Case

  import ExUnit.CaptureIO

  alias Intcode.ExecutionContext
  alias Intcode.ExecutionContext.Adapters.Memory, as: MemoryAdapter

  @longer_part2_example """
                          3,21,1008,21,8,20,1005,20,22,107,8,21,20,1006,20,31,
                          1106,0,36,98,0,0,1002,21,125,20,4,20,1105,1,46,104,
                          999,1105,1,46,1101,1000,1,20,4,20,1105,1,46,98,99
                        """
                        |> Intcode.Program.parse()

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
      assert ExecutionContext.outputs(context) == [888]
    end
  end

  describe "part 1" do
    test "runs the TEST diagnostic program and gets the correct result" do
      assert Day5.Part1.run() == 7_839_346
    end
  end

  describe "part 2 updates" do
    test "position mode - 3,9,8,9,10,9,4,9,99,-1,8 outputs 1 if the input is 8" do
      assert capture_io([input: "8", capture_prompt: false], fn ->
               Intcode.Processor.fix([3, 9, 8, 9, 10, 9, 4, 9, 99, -1, 8])
             end) == "1"
    end

    test "position mode - 3,9,8,9,10,9,4,9,99,-1,8 outputs 0 if the input is not 8" do
      assert capture_io([input: "5", capture_prompt: false], fn ->
               Intcode.Processor.fix([3, 9, 8, 9, 10, 9, 4, 9, 99, -1, 8])
             end) == "0"
    end

    test "immediate mode - 3,3,1108,-1,8,3,4,3,99 outputs 1 if the input is 8" do
      assert capture_io([input: "8", capture_prompt: false], fn ->
               Intcode.Processor.fix([3, 3, 1108, -1, 8, 3, 4, 3, 99])
             end) == "1"
    end

    test "immediate mode - 3,3,1108,-1,8,3,4,3,99 outputs 0 if the input is not 8" do
      assert capture_io([input: "5", capture_prompt: false], fn ->
               Intcode.Processor.fix([3, 3, 1108, -1, 8, 3, 4, 3, 99])
             end) == "0"
    end

    test "larger example - outputs 999 if the input is less than 8" do
      assert capture_io([input: "5", capture_prompt: false], fn ->
               Intcode.Processor.fix(@longer_part2_example)
             end) == "999"
    end

    test "larger example - outputs 1000 if the input is equal to 8" do
      assert capture_io([input: "8", capture_prompt: false], fn ->
               Intcode.Processor.fix(@longer_part2_example)
             end) == "1000"
    end

    test "larger example - outputs 1001 if the input is greater than 8" do
      assert capture_io([input: "9", capture_prompt: false], fn ->
               Intcode.Processor.fix(@longer_part2_example)
             end) == "1001"
    end
  end

  describe "part 2" do
    test "runs the TEST diagnostic program and gets the correct result" do
      assert Day5.Part2.run() == 447803
    end
  end
end

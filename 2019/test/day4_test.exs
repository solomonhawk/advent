defmodule Day4Test do
  use ExUnit.Case

  alias Password.Oracle.Simple, as: SimpleOracle
  alias Password.Oracle.Complex, as: ComplexOracle

  describe "Password.Oracle.Simple.is_candidate/1" do
    test "111111 is a candidate" do
      assert SimpleOracle.is_candidate(111_111) == true
    end

    test "223450 is not a candidate because it has decreasing pairs of digits" do
      assert SimpleOracle.is_candidate(223_450) == false
    end

    test "123789 is not a candidate because it has no matching consecutive digits" do
      assert SimpleOracle.is_candidate(123_789) == false
    end
  end

  describe "Password.Oracle.Simple.candidates/1" do
    test "candidates/1 surfaces all possible viable passwords in the range" do
      assert SimpleOracle.candidates({111_190, 111_222}) == [111_199, 111_222]
    end
  end

  describe "Password.Oracle.Complex.is_candidate/1" do
    test "112233 meets these criteria because the digits never decrease and all repeated digits are exactly two digits long" do
      assert ComplexOracle.is_candidate(112_233) == true
    end

    test "123444 no longer meets the criteria (the repeated 44 is part of a larger group of 444)" do
      assert ComplexOracle.is_candidate(123_444) == false
    end

    test "111122 meets the criteria (even though 1 is repeated more than twice, it still contains a double 22)" do
      assert ComplexOracle.is_candidate(111_122) == true
    end
  end

  describe "Password.Oracle.Complex.candidates/1" do
    test "candidates/1 surfaces all possible viable passwords in the range" do
      assert ComplexOracle.candidates({111_190, 111_222}) == [111_199]
    end
  end

  describe "part 1" do
    test "guesses the correct number of candidates for passwords in the range 357253 - 892942" do
      assert Day4.Part1.run() == 530
    end
  end

  describe "part 2" do
    test "guesses the correct number of candidates for passwords in the range 357253 - 892942" do
      assert Day4.Part2.run() == 324
    end
  end
end

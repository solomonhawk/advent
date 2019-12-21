defmodule Day1Test do
  use ExUnit.Case

  describe "FuelMass Calculator #calculate" do
    test "fuel required for module with mass 12 is 2" do
      assert FuelMass.Calculator.calculate(12) == 2
    end

    test "fuel required for module with mass 14 is 2" do
      assert FuelMass.Calculator.calculate(14) == 2
    end

    test "fuel required for module with mass 1969 is 654" do
      assert FuelMass.Calculator.calculate(1969) == 654
    end

    test "fuel required for module with mass 100756 is 33583" do
      assert FuelMass.Calculator.calculate(100_756) == 33583
    end
  end

  describe "FuelMass Calculator #calculate_total" do
    test "total fuel required for module with mass 14 and its fuel is 2" do
      assert FuelMass.Calculator.calculate_total(14) == 2
    end

    test "total fuel required for module with mass 1969 and its fuel is 966" do
      assert FuelMass.Calculator.calculate_total(1969) == 966
    end

    test "total fuel required for module with mass 100756 and its fuel is 966" do
      assert FuelMass.Calculator.calculate_total(100_756) == 50346
    end
  end

  describe "part 1" do
    test "calculates the sum of all the fuel requirements for the ship modules" do
      assert Day1.Part1.run() == 3_374_289
    end
  end

  describe "part 2" do
    test "calculates the sum of all the fuel requirements for the ship modules and their fuel" do
      assert Day1.Part2.run() == 5_058_559
    end
  end
end

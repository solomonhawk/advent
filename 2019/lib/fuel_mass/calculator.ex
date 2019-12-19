defmodule FuelMass.Calculator do
  def calculate(mass) do
    fuel_required_for_module(mass)
  end

  def calculate_total(mass) do
    total_fuel_required_for_module(mass)
  end

  defp fuel_required_for_module(mass) do
    max(0, Float.floor(mass / 3) - 2)
  end

  defp total_fuel_required_for_module(mass) do
    total_fuel_required_for_module(mass, 0)
  end

  defp total_fuel_required_for_module(mass, acc) when mass <= 0.0 do
    acc |> Kernel.trunc
  end

  defp total_fuel_required_for_module(mass, acc) do
    fuel = fuel_required_for_module(mass)
    total_fuel_required_for_module(fuel, acc + fuel)
  end
end

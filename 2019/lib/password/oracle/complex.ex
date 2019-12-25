defmodule Password.Oracle.Complex do
  import Password.Oracle.Rules,
    only: [
      has_valid_length?: 1,
      has_non_decreasing_digits?: 1,
      has_only_exact_pairs?: 1,
      unit: 1,
      valid?: 1
    ]

  def candidates({min, max}) do
    Enum.filter(min..max, &is_candidate/1)
  end

  def is_candidate(password) do
    password
    |> unit()
    |> has_valid_length?()
    |> has_non_decreasing_digits?()
    |> has_only_exact_pairs?()
    |> valid?()
  end
end

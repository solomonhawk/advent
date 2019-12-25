defmodule Password.Oracle.Simple do
  import Password.Oracle.Rules,
    only: [
      has_valid_length?: 1,
      has_consecutive_duplicates?: 1,
      has_non_decreasing_digits?: 1,
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
    |> has_consecutive_duplicates?()
    |> has_non_decreasing_digits?()
    |> valid?()
  end
end

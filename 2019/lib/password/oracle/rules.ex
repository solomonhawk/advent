defmodule Password.Oracle.Rules do
  defmodule Result do
    defstruct password: nil, valid: true
  end

  def unit(password), do: %Result{password: password}
  def valid?(%Result{valid: valid}), do: valid

  def bind(%Result{valid: false} = result, _), do: result

  def bind(%Result{password: password} = result, func),
    do: %Result{result | valid: apply(func, [password])}

  # It is a six-digit number.
  def has_valid_length?(%Result{} = result), do: bind(result, &has_valid_length/1)

  def has_valid_length(password),
    do: has_valid_length(password, password |> to_string() |> String.length())

  def has_valid_length(_, length) when length == 6, do: true
  def has_valid_length(_, _), do: false

  # Two adjacent digits are the same (like 22 in 122345).
  def has_consecutive_duplicates?(%Result{} = result),
    do: bind(result, &has_consecutive_duplicates/1)

  def has_consecutive_duplicates(password) do
    password
    |> Integer.digits()
    |> Enum.reduce({false, nil}, &is_duplicate_pair/2)
    |> elem(0)
  end

  def is_duplicate_pair(number, {false, nil}), do: {false, number}
  def is_duplicate_pair(number, {false, last_digit}), do: {number == last_digit, number}
  def is_duplicate_pair(_, result), do: result

  # Going from left to right, the digits never decrease; they only ever increase
  # or stay the same (like 111123 or 135679).
  def has_non_decreasing_digits?(%Result{} = result),
    do: bind(result, &has_non_decreasing_digits/1)

  def has_non_decreasing_digits(password) do
    password
    |> Integer.digits()
    |> Enum.reduce({true, 0}, &is_decreasing/2)
    |> elem(0)
  end

  def is_decreasing(number, {true, last_digit}), do: {number >= last_digit, number}
  def is_decreasing(_, result), do: result

  # Two adjacent matching digits are not part of a larger group of matching digits.
  # BUG: this only works when the non-decsending rule is applied
  def has_only_exact_pairs?(%Result{} = result), do: bind(result, &has_only_exact_pairs/1)

  def has_only_exact_pairs(password) do
    password
    |> Integer.digits()
    |> Enum.reduce({nil, Map.new()}, &count_duplicate_digit_occurrences/2)
    |> has_at_least_one_exact_pair()
  end

  def count_duplicate_digit_occurrences(number, {last_digit, occurrences}) do
    map =
      case number == last_digit do
        true -> Map.update(occurrences, number, 2, &(&1 + 1))
        false -> occurrences
      end

    {number, map}
  end

  def count_duplicate_digit_occurrences(_, result), do: result

  def has_at_least_one_exact_pair({_, occurrences}) do
    Enum.any?(occurrences, fn {_, value} -> value == 2 end)
  end
end

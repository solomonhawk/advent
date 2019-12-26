defmodule Intcode.Instruction.Parameter do
  require Intcode.Constants

  import Intcode.Program, only: [value_at: 2]

  alias Intcode.Constants

  defstruct value: nil, mode: Constants.position()

  def new(value: value, mode: mode) do
    %__MODULE__{value: value, mode: mode}
  end

  def to_param({parameter, mode}) do
    new(value: parameter, mode: mode)
  end

  def deref(program, %__MODULE__{value: position, mode: Constants.position()}) do
    value_at(program, position)
  end

  def deref(_, %__MODULE__{value: value, mode: Constants.immediate()}), do: value

  def value_of(%__MODULE__{value: value}), do: value
end

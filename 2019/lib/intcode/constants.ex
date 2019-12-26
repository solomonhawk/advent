defmodule Intcode.Constants do
  use Constants

  require Intcode.OpCodes

  alias Intcode.OpCodes

  @position 0
  @immediate 1

  define(position, @position)
  define(immediate, @immediate)

  def instruction_length(OpCodes.add()), do: 4
  def instruction_length(OpCodes.mult()), do: 4
  def instruction_length(OpCodes.input()), do: 2
  def instruction_length(OpCodes.output()), do: 2
  def instruction_length(OpCodes.halt()), do: 1
end

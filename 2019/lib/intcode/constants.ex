defmodule Intcode.Constants do
  use Constants

  require Intcode.OpCodes

  alias Intcode.OpCodes

  @position 0
  @immediate 1

  define(position, @position)
  define(immediate, @immediate)

  def inst_len(OpCodes.add()), do: 4
  def inst_len(OpCodes.mult()), do: 4
  def inst_len(OpCodes.input()), do: 2
  def inst_len(OpCodes.output()), do: 2
  def inst_len(OpCodes.jump_if_true()), do: 3
  def inst_len(OpCodes.jump_if_false()), do: 3
  def inst_len(OpCodes.less_than()), do: 4
  def inst_len(OpCodes.equals()), do: 4
  def inst_len(OpCodes.halt()), do: 1
end

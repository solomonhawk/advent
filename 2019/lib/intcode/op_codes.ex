defmodule Intcode.OpCodes do
  use Constants

  @add 1
  @mult 2
  @input 3
  @output 4
  @jump_if_true 5
  @jump_if_false 6
  @less_than 7
  @equals 8
  @halt 99

  define(add, @add)
  define(mult, @mult)
  define(input, @input)
  define(output, @output)
  define(jump_if_true, @jump_if_true)
  define(jump_if_false, @jump_if_false)
  define(less_than, @less_than)
  define(equals, @equals)
  define(halt, @halt)

  def op_type(@add), do: :add
  def op_type(@mult), do: :mult
  def op_type(@input), do: :input
  def op_type(@output), do: :output
  def op_type(@jump_if_true), do: :jump_if_true
  def op_type(@jump_if_false), do: :jump_if_false
  def op_type(@less_than), do: :less_than
  def op_type(@equals), do: :equals
  def op_type(@halt), do: :halt
end

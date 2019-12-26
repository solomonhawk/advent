defmodule Intcode.OpCodes do
  use Constants

  @add 1
  @mult 2
  @input 3
  @output 4
  @halt 99

  define(add, @add)
  define(mult, @mult)
  define(input, @input)
  define(output, @output)
  define(halt, @halt)

  def op_type(@add), do: :add
  def op_type(@mult), do: :mult
  def op_type(@input), do: :input
  def op_type(@output), do: :output
  def op_type(@halt), do: :halt
end

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
end

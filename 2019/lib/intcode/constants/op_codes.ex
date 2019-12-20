defmodule Intcode.Constants.OpCodes do
  use Constants

  @add 1
  @mult 2
  @halt 99

  define add, @add
  define mult, @mult
  define halt, @halt
end

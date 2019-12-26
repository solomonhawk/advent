defprotocol Adapter do
  @doc "Requests user input from the adapter"
  def request_input(adapter)

  @doc "Requests the adapter to output a value"
  def request_output(adapter, output)

  @doc "Collects the outputs for an adapter"
  def outputs(adapter)
end

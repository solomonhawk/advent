defmodule Amplifier.Configuration do
  defstruct mode: :series, phase_settings: nil

  @type mode :: :series | :loop

  @type t :: %__MODULE__{
          mode: mode(),
          phase_settings: list(integer())
        }

  def new(attrs \\ []) do
    struct(__MODULE__, attrs)
    |> set_phase_settings
  end

  def set_phase_settings(config) do
    phase_settings =
      case config.mode do
        :series -> [0, 1, 2, 3, 4]
        :loop -> [9, 7, 8, 5, 6]
      end

    struct(config, phase_settings: phase_settings)
  end
end

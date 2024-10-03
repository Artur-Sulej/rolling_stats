defmodule RollingStats do
  @moduledoc """
  RollingStats is an interface to business logic for rolling statistics.
  """

  alias RollingStats.StatsData

  def get_stats(symbol, exponent) do
    case RollingStats.Native.get_stats(symbol, exponent) do
      nil -> {:error, :not_found}
      stats_data = %StatsData{} -> {:ok, stats_data}
    end
  end

  def add_batch(symbol, values) do
    RollingStats.Native.add_items(symbol, values)
    :ok
  end
end

defmodule RollingStatsWeb.StatsJSON do
  alias RollingStats.StatsData

  def stats_data(%{stats_data: %StatsData{} = stats_data}) do
    Map.from_struct(stats_data)
  end
end

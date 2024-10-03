defmodule RollingStatsWeb.StatsController do
  use RollingStatsWeb, :controller

  action_fallback RollingStatsWeb.FallbackController

  def get_stats(conn, params) do
    with {:ok, %{symbol: symbol, exponent: exponent}} <- parse_get_stats_params(params),
         {:ok, stats_data} <- RollingStats.get_stats(symbol, exponent) do
      render(conn, :stats_data, stats_data: stats_data)
    else
      {:error, :invalid_parameters} -> {:error, :invalid_parameters}
      {:error, :not_found} -> {:error, :not_found}
    end
  end

  def add_batch(conn, %{"symbol" => symbol, "values" => values}) when is_list(values) do
    with :ok <- RollingStats.add_batch(symbol, values) do
      send_resp(conn, :created, "")
    end
  end

  defp parse_get_stats_params(%{"symbol" => symbol, "k" => k}) do
    exponent = String.to_integer(k)

    if exponent >= 1 && exponent <= 8 do
      {:ok, %{symbol: symbol, exponent: exponent}}
    else
      {:error, :invalid_parameters}
    end
  rescue
    ArgumentError -> {:error, :invalid_parameters}
  end

  defp parse_get_stats_params(_) do
    {:error, :invalid_parameters}
  end
end

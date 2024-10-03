defmodule RollingStatsWeb.FallbackController do
  @moduledoc """
  Translates controller action results into valid `Plug.Conn` responses.

  See `Phoenix.Controller.action_fallback/1` for more details.
  """
  use RollingStatsWeb, :controller

  # This clause is an example of how to handle resources that cannot be found.
  def call(conn, {:error, :not_found}) do
    conn
    |> put_status(:not_found)
    |> put_view(html: RollingStatsWeb.ErrorHTML, json: RollingStatsWeb.ErrorJSON)
    |> render(:"404")
  end

  def call(conn, {:error, :invalid_parameters}) do
    conn
    |> put_status(:bad_request)
    |> put_view(html: RollingStatsWeb.ErrorHTML, json: RollingStatsWeb.ErrorJSON)
    |> render(:"400")
  end
end

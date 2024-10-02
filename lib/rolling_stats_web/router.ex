defmodule RollingStatsWeb.Router do
  use RollingStatsWeb, :router

  pipeline :api do
    plug :accepts, ["json"]
  end

  scope "/api", RollingStatsWeb do
    pipe_through :api
  end
end

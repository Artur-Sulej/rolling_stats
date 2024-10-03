defmodule RollingStatsWeb.Router do
  use RollingStatsWeb, :router

  pipeline :api do
    plug :accepts, ["json"]
  end

  scope "/", RollingStatsWeb do
    pipe_through :api

    post "/add_batch", StatsController, :add_batch
    get "/stats", StatsController, :get_stats
  end
end

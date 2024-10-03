defmodule RollingStatsWeb.StatsControllerTest do
  use RollingStatsWeb.ConnCase

  setup %{conn: conn} do
    {:ok, conn: put_req_header(conn, "accept", "application/json")}
  end

  test "adds a batch and fetches stats", %{conn: conn} do
    batch_payload = %{
      "symbol" => "A",
      "values" => [55.55, 34.67, 23.01, 88.88]
    }

    conn = post(conn, "/add_batch", batch_payload)
    assert response(conn, 201)

    conn = get(conn, "/stats?symbol=A&k=2")

    assert %{
             "average" => 50.5275,
             "last" => 88.88,
             "max" => 88.88,
             "min" => 23.01,
             "variance" => 626.2032187499995
           } = json_response(conn, 200)
  end
end

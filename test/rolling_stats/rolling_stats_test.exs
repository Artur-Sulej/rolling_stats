defmodule RollingStatsTest do
  use ExUnit.Case

  alias RollingStats.StatsData

  test "adds a batch and fetches stats" do
    assert :ok ==
             RollingStats.add_batch(
               "B",
               [999_999.0, 10.01, 20.02, 30.03, 40.04, 50.05, 60.06, 70.07, 80.08, 90.09, 100.10]
             )

    assert {:ok, stats_1} = RollingStats.get_stats("B", 1)
    assert {:ok, stats_2} = RollingStats.get_stats("B", 2)
    assert {:ok, stats_3} = RollingStats.get_stats("B", 3)
    assert {:ok, stats_4} = RollingStats.get_stats("B", 4)
    assert {:ok, stats_5} = RollingStats.get_stats("B", 5)
    assert {:ok, stats_6} = RollingStats.get_stats("B", 6)
    assert {:ok, stats_7} = RollingStats.get_stats("B", 7)
    assert {:ok, stats_8} = RollingStats.get_stats("B", 8)

    assert stats_1 == %StatsData{
             average: 55.055000000006984,
             last: 100.1,
             max: 100.1,
             min: 10.01,
             variance: 826.6508314445425
           }

    assert stats_2 == %StatsData{
             average: 90959.05,
             last: 100.1,
             max: 999_999.0,
             min: 10.01,
             variance: 82_635_363_821.10101
           }

    assert stats_3 == %StatsData{
             average: 90959.05,
             last: 100.1,
             max: 999_999.0,
             min: 10.01,
             variance: 82_635_363_821.10101
           }

    assert stats_4 == %StatsData{
             average: 90959.05,
             last: 100.1,
             max: 999_999.0,
             min: 10.01,
             variance: 82_635_363_821.10101
           }

    assert stats_5 == %StatsData{
             average: 90959.05,
             last: 100.1,
             max: 999_999.0,
             min: 10.01,
             variance: 82_635_363_821.10101
           }

    assert stats_6 == %StatsData{
             average: 90959.05,
             last: 100.1,
             max: 999_999.0,
             min: 10.01,
             variance: 82_635_363_821.10101
           }

    assert stats_7 == %StatsData{
             average: 90959.05,
             last: 100.1,
             max: 999_999.0,
             min: 10.01,
             variance: 82_635_363_821.10101
           }

    assert stats_8 == %StatsData{
             average: 90959.05,
             last: 100.1,
             max: 999_999.0,
             min: 10.01,
             variance: 82_635_363_821.10101
           }
  end

  test "same result for one and multiple batches" do
    assert :ok == RollingStats.add_batch("C", [8937.36, 94.73])
    assert :ok == RollingStats.add_batch("D", [8937.36])
    assert :ok == RollingStats.add_batch("D", [94.73])

    assert {:ok, stats_a} = RollingStats.get_stats("C", 1)
    assert {:ok, stats_b} = RollingStats.get_stats("D", 1)

    assert stats_a == stats_b
  end

  test "error for symbol without values" do
    assert {:error, :not_found} == RollingStats.get_stats("E", 1)
    assert {:error, :not_found} == RollingStats.get_stats("E", 2)
  end

  test "error for exponent too high" do
    assert :ok == RollingStats.add_batch("F", [123.456])
    assert {:ok, _} = RollingStats.get_stats("F", 8)
    assert {:error, :not_found} == RollingStats.get_stats("F", 9)
  end
end

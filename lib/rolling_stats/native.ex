defmodule RollingStats.Native do
  use Rustler, otp_app: :rolling_stats, crate: "rolling_stats"

  # When your NIF is loaded, it will override this function.
  def add_items(_identifier, _items), do: :erlang.nif_error(:nif_not_loaded)
  def get_stats(_identifier, _exponent), do: :erlang.nif_error(:nif_not_loaded)
end

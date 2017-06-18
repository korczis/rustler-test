defmodule Calculator.Native do
  use Rustler, otp_app: :rustler_test, crate: :calculator

  # When your NIF is loaded, it will override this function.
  def add(_a, _b), do: throw :nif_not_loaded
  def sub(_a, _b), do: throw :nif_not_loaded
end

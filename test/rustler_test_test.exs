defmodule RustlerTestTest do
  use ExUnit.Case
  doctest RustlerTest

  test "add" do
    assert Calculator.Native.add(1, 2) == {:ok, 3}
  end

  test "sub" do
    assert Calculator.Native.sub(3, 2) == {:ok, 1}
  end
end

defmodule Secrets do
  def secret_add(secret) do
    &(&1 + secret)
  end

  def secret_subtract(secret) do
    &(&1 - secret)
  end

  def secret_multiply(secret) do
    &(&1 * secret)
  end

  def secret_divide(secret) do
    &(div(&1, secret))
  end

  def secret_and(secret) do
    return_function = fn param ->
      Bitwise.band(param, secret)
    end
    return_function
  end

  def secret_xor(secret) do
    return_function = fn param ->
      Bitwise.bxor(param, secret)
    end
    return_function
  end

  def secret_combine(secret_function1, secret_function2) do
    return_function = fn param ->
      secret_function2.(secret_function1.(param))
    end
    return_function
  end
end

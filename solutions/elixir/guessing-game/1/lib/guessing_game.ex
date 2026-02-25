defmodule GuessingGame do
  def compare(secret_number, guess) when is_number(guess) do
        cond do
      secret_number == guess ->
        "Correct"
      secret_number == guess + 1 or secret_number == guess - 1 ->
        "So close"
      secret_number > guess ->
        "Too low"
      secret_number < guess ->
        "Too high"
    end
  end

  def compare(secret_number, guess \\ :no_guess) do
    "Make a guess"
  end
end

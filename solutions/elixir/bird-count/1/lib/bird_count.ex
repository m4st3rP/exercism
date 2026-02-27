defmodule BirdCount do
  def today(list) do
    List.first(list)
  end

  def increment_day_count([]) do
    [1]
  end

  def increment_day_count([head | tail]) do
    [head + 1 | tail]
  end

  def has_day_without_birds?([]), do: false

  def has_day_without_birds?([0 | _]), do: true

  def has_day_without_birds?([_ | tail]) do
    has_day_without_birds?(tail)
  end

  def total([]) do
    0
  end

  def total([head | tail]) do
    head + total(tail)
  end

  def busy_days([]) do
    0
  end

  def busy_days([head | tail]) when head >= 5 do
    1 + busy_days(tail)
  end

  def busy_days([head | tail]) do
    0 + busy_days(tail)
  end
end

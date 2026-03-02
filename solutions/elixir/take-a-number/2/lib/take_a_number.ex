defmodule TakeANumber do
  defp send_state(sender_pid, ticket_number, number_modifier) do
    send(sender_pid, ticket_number + number_modifier)
    loop(ticket_number + number_modifier)
  end

  defp loop(ticket_number) do
    receive do
      {:report_state, sender_pid} -> send_state(sender_pid, ticket_number, 0)
      {:take_a_number, sender_pid} -> send_state(sender_pid, ticket_number, 1)
      :stop -> nil
      _ -> loop(ticket_number)
    end
  end
  
  def start() do
    spawn(fn -> loop(0) end)
  end
end

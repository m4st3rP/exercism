defmodule Username do
  def sanitize([]) do
    ~c""
  end
  
  def sanitize([head | tail]) do
    case head do
      head when ?a <= head and head <= ?z -> [head | sanitize(tail)]
      ?_ -> [head | sanitize(tail)]
      ?ä -> [?a, ?e | sanitize(tail)]
      ?ö -> [?o, ?e | sanitize(tail)]
      ?ü -> [?u, ?e | sanitize(tail)]
      ?ß -> [?s, ?s | sanitize(tail)]
      _ -> sanitize(tail)
    end
  end
end

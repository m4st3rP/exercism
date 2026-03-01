defmodule NameBadge do
  def print(id, name, department) do
    department_string = if department do
      String.upcase(department)
    else
      "OWNER"
    end
    
    if id do
      "[#{id}] - #{name} - #{department_string}"
    else 
      "#{name} - #{department_string}"
    end
  end
end

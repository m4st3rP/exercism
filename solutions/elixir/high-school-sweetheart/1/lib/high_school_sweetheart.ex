defmodule HighSchoolSweetheart do
  def first_letter(name) do
    name |> String.trim() |> String.first()
  end

  def initial(name) do
    first_letter(name) |> String.upcase() |> Kernel.<>(".")
  end

  def initials(full_name) do
    [forename, surname] = String.split(full_name)
    initial(forename) <> " " <> initial(surname)
  end

  def pair(full_name1, full_name2) do
    # ❤-------------------❤
    # |  X. X.  +  X. X.  |
    # ❤-------------------❤
    first_name = initials(full_name1)
    second_name = initials(full_name2)
    
    """
    ❤-------------------❤
    |  #{first_name}  +  #{second_name}  |
    ❤-------------------❤
    """

    
  end
end

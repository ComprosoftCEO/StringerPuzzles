require("functions")

function generateTestCase()
  local input = randomABCString(2, 13);

  local output = ""
  for i = 1, #input do
    if (i % 2) == 0 then
      output = output .. input:sub(i, i)
    end
  end

  return input, output
end

require("functions")

function generateTestCase()
  local input = randomABCString(1, 10);
  local output = input:gsub("b", "a"):gsub("c", "b", 1)

  return input, output
end

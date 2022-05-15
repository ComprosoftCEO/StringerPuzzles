require("functions")

function generateTestCase()
  local input = randomABCString(1, 12);
  local output = input:gsub("a+", "a"):gsub("b+", "b"):gsub("c+", "c")

  return input, output
end

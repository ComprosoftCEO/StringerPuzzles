require("functions")

function generateTestCase()
  local input = randomABCString(3, 10);
  local output = input:gsub("c", "A"):gsub("b", "c"):gsub("a", "b"):gsub("A", "a")
  return input, output
end

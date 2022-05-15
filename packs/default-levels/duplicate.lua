require("functions")

function generateTestCase()
  local input = randomABCString(1, 5);
  local output = input:gsub("a", "aa"):gsub("b", "bb"):gsub("c", "cc")

  return input, output
end

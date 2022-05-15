require("functions")

function generateTestCase()
  local input = randomABCString(3, 12);
  local output = input:gsub("^a+", ""):gsub("a+$", "")
  return input, output
end

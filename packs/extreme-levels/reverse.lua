require("functions")

function generateTestCase()
  local input = randomABCString(1, 7);
  local output = input:reverse()

  return input, output
end

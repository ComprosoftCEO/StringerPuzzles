require("functions")

function generateTestCase()
  local input = randomABCString(3, 11);
  local output = input:sub(4)

  return input, output
end

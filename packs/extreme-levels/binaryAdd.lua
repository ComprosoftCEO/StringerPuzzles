require("functions")

function generateTestCase()
  local a = math.random(1, 7)
  local b = math.random(1, 7)

  local input = toBinary(a) .. "+" .. toBinary(b)
  local output = toBinary(a + b)

  return input, output
end

require("functions")

function generateTestCase()
  local a = math.random(1, 16)

  local input = ("1"):rep(a)
  local output = toBinary(a)

  return input, output
end

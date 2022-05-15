require("functions")

function generateTestCase()
  local a = math.random(1, 7)
  local b = math.random(1, 7)

  local input = ("1"):rep(a) .. "+" .. ("1"):rep(b)
  local output = ("1"):rep(a + b)

  return input, output
end

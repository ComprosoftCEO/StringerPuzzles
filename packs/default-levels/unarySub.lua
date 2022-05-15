require("functions")

function generateTestCase()
  local one = math.random(1, 7)
  local two = math.random(1, 7)

  local a = math.max(one, two)
  local b = math.min(one, two)
  if a == b then
    a = a + 1
  end

  local input = ("1"):rep(a) .. "-" .. ("1"):rep(b)
  local output = ("1"):rep(a - b)

  return input, output
end

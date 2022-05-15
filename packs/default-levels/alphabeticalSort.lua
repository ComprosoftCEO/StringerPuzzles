require("functions")

function generateTestCase()
  local a = math.random(1, 5)
  local b = math.random(1, 5)
  local c = math.random(1, 5)

  local output = ("a"):rep(a) .. ("b"):rep(b) .. ("c"):rep(c)
  local input = shuffle(output)

  return input, output
end

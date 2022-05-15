require("functions")

function generateTestCase()
  local a = math.random(1, 5)

  local b = a
  while b == a do
    b = math.random(1, 5)
  end

  local c = a
  while c == a or c == b do
    c = math.random(1, 5)
  end

  local input = shuffle(("a"):rep(a) .. ("b"):rep(b) .. ("c"):rep(c))

  local output
  local max = math.max(a, b, c)
  if max == a then output = "a"
  elseif max == b then output = "b"
  else output = "c" end

  return input, output
end

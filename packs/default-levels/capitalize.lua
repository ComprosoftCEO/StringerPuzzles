require("functions")

function generateTestCase()
  local str = randomABCString(1, 7);
  return str, string.upper(str)
end

require("functions")

function generateTestCase()
  local input = randomABCString(1, 11);

  -- Might need to append a character if the length is even
  if (#input % 2) == 0 then
    input = input .. randomABC()
  end

  local center = math.ceil(#input / 2)
  local output = input:sub(1, center - 1) .. input:sub(center + 1, #input)

  return input, output
end

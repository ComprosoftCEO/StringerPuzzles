require("functions")

function generateTestCase()
  local input = randomABCString(1, 10);

  local first = input:sub(1, 1)
  local last = input:sub(#input, #input)

  local output
  if #input > 1 then
    output = last .. input:sub(2, #input - 1) .. first
  else
    output = first
  end


  return input, output
end

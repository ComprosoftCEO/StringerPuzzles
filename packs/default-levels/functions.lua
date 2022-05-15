--[[
  Define some useful helper functions for writing the code
]]

-- Shuffle the given input string
function shuffle(str)
  local letters = {}
  for letter in str:gmatch '.[\128-\191]*' do
    table.insert(letters, { letter = letter, rnd = math.random() })
  end
  table.sort(letters, function(a, b) return a.rnd < b.rnd end)
  for i, v in ipairs(letters) do letters[i] = v.letter end
  return table.concat(letters)
end

-- Convert to a binary number with a minimum number of bits
function toBinary(num, bits)
  -- returns a table of bits, most significant first.
  bits = bits or math.max(1, select(2, frexp(num)))
  local t = {} -- will contain the bits
  for b = bits, 1, -1 do
    t[b] = math.fmod(num, 2)
    num = math.floor((num - t[b]) / 2)
  end
  return table.concat(t)
end

-- math.frexp() replacement for Lua 5.3 when compiled without LUA_COMPAT_MATHLIB.
-- The C approach is just to type-pun the float, but we can't do that here short
-- of stupid loadstring() tricks, which would be both architecture and version
-- dependent and a maintenance headache at best. So instead we use math.
function frexp(x)
  if x == 0 then return 0.0, 0.0 end
  local e = math.floor(math.log(math.abs(x)) / math.log(2))
  if e > 0 then
    -- Why not x / 2^e? Because for large-but-still-legal values of e this
    -- ends up rounding to inf and the wheels come off.
    x = x * 2 ^ -e
  else
    x = x / 2 ^ e
  end
  -- Normalize to the range [0.5,1)
  if math.abs(x) >= 1.0 then
    x, e = x / 2, e + 1
  end
  return x, e
end

-- Generate a string with a random length between [min, max]
--  containing letters a, b, and c
function randomABCString(min, max)
  local result = ""
  for i = 1, math.random(min, max) do
    result = result .. randomABC()
  end

  return result
end

-- Randomly return either "a", "b", or "c"
function randomABC()
  local ALL_LETTERS = { "a", "b", "c" }
  return ALL_LETTERS[math.random(#ALL_LETTERS)]
end

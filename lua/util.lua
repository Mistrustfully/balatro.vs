local util = {}

---Takes a reference function and a hook function and combines them.
---@param ref function
---@param hook function
---@return function
function util.hooked(ref, hook)
    return function(...)
        ref(...)
        hook(...)
    end
end

---Same as util.hooked except the hook gets called before the ref.
---@param ref function
---@param hook function
---@return function
function util.prehooked(ref, hook)
    return function(...)
        hook(...)
        ref(...)
    end
end

---Serializes param `t` into length delimited json string.
---`skip_header` will skip adding the length header.
---@param t any
---@param skip_header? boolean
---@return string
function util.ser(t, skip_header)
    if not skip_header then
        return love.data.pack("string", ">s2", util.ser(t, true))
    end

    if type(t) == "table" then
        local str = "{"

        -- Recursively call for the key and value
        for i, v in pairs(t) do
            str = str .. string.format(
                "%s:%s,",
                util.ser(i, true),
                util.ser(v, true)
            )
        end

        -- Trim trailing comma
        str = str:sub(1, -2)
        str = str .. "}"

        return str
    elseif type(t) == "string" then
        return string.format([["%s"]], t)
    elseif type(t) == "number" or type(t) == "boolean" then
        return tostring(t)
    end

    -- Default string if it's not a datatype we can send
    return [["<invalid-type>"]]
end

---Deserializes JSON `t` into a lua table
---@param t string
---@return table
function util.de(t)
    --- todo
    return t
end

return util

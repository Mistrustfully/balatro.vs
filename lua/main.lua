local util = require("util")

local packetChannel = love.thread.getChannel("packet")
local socketChannel = love.thread.getChannel("socket")

local newThread = love.thread.newThread("Mods/balatro.vs/net.lua")

love.load = util.hooked(love.load, function(...)
    newThread:start("127.0.0.1", 8080, G.PROFILES[G.SETTINGS.profile].name)
end)

love.update = util.hooked(love.update, function(...)
    local packet = packetChannel:pop()

    if packet then
        print(packet)
    end
end)

love.quit = util.prehooked(love.quit, function(...)
    socketChannel:push("close")
end)

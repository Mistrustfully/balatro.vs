local util = require("util")
local socket = require("socket")

local function createClient(addr, port, name)
    local client = socket.tcp()
    local success, err = client:connect(addr, port)

    if success ~= 1 then
        print("Error connecting to server!")
        print(err)
    else
        client:send(util.ser({ name = name }))
        return client
    end
end

local function readPacket(client)
    local byte_header = client:receive(2)
    local byte_length = love.data.unpack(">I2", byte_header)
 
    local string = client:receive(tonumber(byte_length))
    return util.de(string)
end

local client = createClient(...)
local packetChannel = love.thread.getChannel("packet")
local socketChannel = love.thread.getChannel("socket")

while true do
    local inst = socketChannel:pop()

    if inst == "close" then
        client:close()
        return
    end

    if client then
        packetChannel:push(readPacket(client))
    end

    socket.sleep(0.1)
end

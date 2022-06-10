(function ()
    local r = {};
    r.name = "Clarketm ProxyList";
    r.source = "https://github.com/clarketm/proxy-list";
    r.direct = "https://raw.githubusercontent.com/clarketm/proxy-list/master/proxy-list-raw.txt";
    r.addresses = { };
    r.valid = false;

    -- network request
    local response = get(r.direct);

    if response.status == 200 and response.text ~= nil then
        r.valid = true;
        for line in response.text:gmatch("[^\r\n]+") do
            table.insert(r.addresses, line);
        end
    end

    return r;
end)();
(function ()
    local r = {};
    r.name = "geonode-proxylist";
    r.source = "https://proxylist.geonode.com/";
    r.direct = "https://proxylist.geonode.com/api/proxy-list?limit=20000&page=1&sort_by=lastChecked&sort_type=desc";
    r.addresses = { };
    r.valid = false;

    -- network request
    local response = get(r.direct);

    if response.status == 200 and response.text ~= nil then
        r.valid = true;
        local object = json.decode(response.text);
        for _, v in ipairs(object.data) do
            table.insert(r.addresses, v.ip);
        end
    end

    return r;
end)()
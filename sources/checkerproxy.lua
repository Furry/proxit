(function ()
    local r = {};
    r.name = "Checkerproxy";
    r.source = "https://checkerproxy.net/";
    r.direct = "https://checkerproxy.net/api/archive/";
    r.addresses = { };
    r.valid = false;

    local archiveListResponse = get(r.direct);
    if archiveListResponse.status == 200 and archiveListResponse.text ~= nil then
        local listObj = json.decode(archiveListResponse.text);

        -- The archive list looks like this, with the previous x number of dates. We'll iterate and scrape every one.
        -- [{"date":"2022-06-02","proxies":9517},{"date":"2022-06-03","proxies":5116},{"date":"2022-06-04","proxies":235}]
        for _, v in ipairs(listObj) do
            local date = v.date;
        end
    end
    return r;
end)()
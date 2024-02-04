# OPNsense API notes

Info captured from the OPNsense GUI.

`/api/unbound/settings/searchHostOverride/`: Request body

```json
{"current":1,"rowCount":-1,"sort":{"hostname":"asc"},"searchPhrase":""}
```

Response body

```json
{"rows":[{"uuid":"062703d3-f02c-41c1-befc-ba2e5b3731c3","enabled":"1","hostname":"somehost","domain":"example.com","rr":"A (IPv4 address)","mxprio":"","mx":"","server":"0.0.0.0","description":"foo"}],"rowCount":1,"total":1,"current":1}
```

`/api/unbound/settings/getHostOverride/`: No request body/headers/params

Response body

```json
{"host":{"enabled":"1","hostname":"","domain":"","rr":{"A":{"value":"A (IPv4 address)","selected":1},"AAAA":{"value":"AAAA (IPv6 address)","selected":0},"MX":{"value":"MX (Mail server)","selected":0}},"mxprio":"","mx":"","server":"","description":""}}
```

`/api/unbound/settings/getHostOverride/275defbd-f239-494c-998b-271cccda5ab4?fetchmode=copy`: No request body/headers/params

Response body

```json
{"host":{"enabled":"1","hostname":"foo","domain":"bar","rr":{"A":{"value":"A (IPv4 address)","selected":0},"AAAA":{"value":"AAAA (IPv6 address)","selected":1},"MX":{"value":"MX (Mail server)","selected":0}},"mxprio":"","mx":"","server":"fe80::","description":"baz"}}
```

`/api/unbound/settings/addHostOverride/`: Request body

```json
{"host":{"enabled":"1","hostname":"somehost","domain":"example.com","rr":"A","mxprio":"","mx":"","server":"0.0.0.0","description":"foo"}}
````

```json
{"host":{"enabled":"1","hostname":"foo","domain":"bar","rr":"AAAA","mxprio":"","mx":"","server":"fe80::","description":"baz"}}
```

Response body

```json
{"result":"saved","uuid":"062703d3-f02c-41c1-befc-ba2e5b3731c3"}
```

`/api/unbound/settings/delHostOverride/062703d3-f02c-41c1-befc-ba2e5b3731`: POST but empty payload

Response body

```json
{"result":"deleted"}
```

Request headers

```http
POST /api/unbound/settings/addHostOverride/ HTTP/2
Host: 192.168.1.1
User-Agent: Mozilla/5.0 (Windows NT 10.0; rv:121.0) Gecko/20100101 Firefox/121.0
Accept: application/json, text/javascript, */*; q=0.01
Accept-Language: en-US,en;q=0.5
Accept-Encoding: gzip, deflate, br
Referer: https://192.168.1.1/ui/unbound/overrides
Content-Type: application/json
X-CSRFToken: d2NJMXJzNlRvYjh6WTBCZmdkQkpHZz09
X-Requested-With: XMLHttpRequest
Content-Length: 137
Origin: https://192.168.1.1
DNT: 1
Sec-GPC: 1
Connection: keep-alive
Cookie: PHPSESSID=b3d4fa1601dad3b00c1da6b0c76b6c88; cookie_test=cd09b833bfad789608deb1c13b2fed9d
Sec-Fetch-Dest: empty
Sec-Fetch-Mode: cors
Sec-Fetch-Site: same-origin
TE: trailers
```


#!/usr/bin/env python3
import http.server

port=8000
print("Porous absorber test webserver listening on port %d" % port)

http.server.SimpleHTTPRequestHandler.extensions_map['.wasm'] = 'application/wasm'

httpd = http.server.HTTPServer(('localhost', port), http.server.SimpleHTTPRequestHandler)

print( "Mapping \".wasm\" to \"%s\"" % http.server.SimpleHTTPRequestHandler.extensions_map['.wasm'])

httpd.serve_forever()

#!/usr/bin/env python3
"""Simple HTTP server with no-cache headers for development"""
import http.server
import socketserver
from pathlib import Path

class NoCacheHTTPRequestHandler(http.server.SimpleHTTPRequestHandler):
    def end_headers(self):
        # Add no-cache headers
        self.send_header('Cache-Control', 'no-store, no-cache, must-revalidate, max-age=0')
        self.send_header('Pragma', 'no-cache')
        self.send_header('Expires', '0')
        super().end_headers()
    
    def log_message(self, format, *args):
        # Log to stdout
        print(f"{self.address_string()} - [{self.log_date_time_string()}] {format % args}")

PORT = 8000
Handler = NoCacheHTTPRequestHandler

with socketserver.TCPServer(("0.0.0.0", PORT), Handler) as httpd:
    print(f"Server running at http://0.0.0.0:{PORT}/")
    print("Press Ctrl+C to stop")
    httpd.serve_forever()

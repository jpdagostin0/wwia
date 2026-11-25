#!/usr/bin/env python3
"""Minimal RCON client for the dev server (tools/rcon.py 'setblock ...')."""
import socket
import struct
import sys


def packet(req_id, kind, body):
    data = struct.pack("<ii", req_id, kind) + body.encode() + b"\0\0"
    return struct.pack("<i", len(data)) + data


def read(sock):
    length = struct.unpack("<i", sock.recv(4))[0]
    data = b""
    while len(data) < length:
        data += sock.recv(length - len(data))
    req_id, _kind = struct.unpack("<ii", data[:8])
    return req_id, data[8:-2].decode(errors="replace")


def main():
    password = "wwia"
    with socket.create_connection(("127.0.0.1", 25575), timeout=10) as sock:
        sock.sendall(packet(1, 3, password))
        if read(sock)[0] == -1:
            sys.exit("rcon auth failed")
        for command in sys.argv[1:]:
            sock.sendall(packet(2, 2, command))
            print(read(sock)[1])


if __name__ == "__main__":
    main()

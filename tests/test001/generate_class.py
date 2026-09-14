"""Generate ldc, ldc_w, and ldc2_w explicitly; Main.java documents the output.

ldc_w permits any valid 16-bit pool index, even one that also fits in ldc.
Long/double constants reserve two pool slots, represented by empty entries here.
"""
from pathlib import Path
import struct


def u2(value):
    return struct.pack('>H', value)


def u4(value):
    return struct.pack('>I', value)


def utf8(text):
    data = text.encode('utf-8')
    return b'\x01' + u2(len(data)) + data


pool = [
    utf8('Main'),                              # 1
    b'\x07' + u2(1),                           # 2: Class Main
    utf8('java/lang/Object'),                  # 3
    b'\x07' + u2(3),                           # 4: Class Object
    utf8('main'),                              # 5
    utf8('([Ljava/lang/String;)V'),            # 6
    utf8('Code'),                              # 7
    utf8('java/lang/System'),                  # 8
    b'\x07' + u2(8),                           # 9: Class System
    utf8('out'),                               # 10
    utf8('Ljava/io/PrintStream;'),             # 11
    b'\x0c' + u2(10) + u2(11),                 # 12: NameAndType out
    b'\x09' + u2(9) + u2(12),                  # 13: Fieldref System.out
    utf8('java/io/PrintStream'),               # 14
    b'\x07' + u2(14),                          # 15: Class PrintStream
    utf8('println'),                           # 16
    utf8('(I)V'),                              # 17
    b'\x0c' + u2(16) + u2(17),                 # 18: NameAndType println
    b'\x0a' + u2(15) + u2(18),                 # 19: Methodref println
]

pool.extend([
    utf8('hi'),                               # 20
    b'\x08' + u2(20),                          # 21: String hi
    b'\x05' + struct.pack('>q', 42),            # 22: Long
    b'',                                      # 23: reserved (no bytes)
    b'\x06' + struct.pack('>d', 2.5),           # 24: Double
    b'',                                      # 25: reserved (no bytes)
    utf8('(J)V'),                              # 26
    b'\x0c' + u2(16) + u2(26),                 # 27
    b'\x0a' + u2(15) + u2(27),                 # 28: println(long)
    utf8('(D)V'),                              # 29
    b'\x0c' + u2(16) + u2(29),                 # 30
    b'\x0a' + u2(15) + u2(30),                 # 31: println(double)
    utf8('(Ljava/lang/String;)V'),            # 32
    b'\x0c' + u2(16) + u2(32),                 # 33
    b'\x0a' + u2(15) + u2(33),                 # 34: println(String)
])
get_stdout = bytes([0xb2, 0x00, 0x0d])
code = b''
for load, method in [
    (bytes([0x12, 21]), 34),                   # ldc
    (bytes([0x13, 0, 21]), 34),                # ldc_w
    (bytes([0x14, 0, 22]), 28),                # ldc2_w long
    (bytes([0x14, 0, 24]), 31),                # ldc2_w double
]:
    code += get_stdout + load + bytes([0xb6]) + u2(method)
code += bytes([0xb1])                         # return

code_attribute = u2(3) + u2(1) + u4(len(code)) + code + u2(0) + u2(0)
main_method = u2(0x0009) + u2(5) + u2(6) + u2(1)
main_method += u2(7) + u4(len(code_attribute)) + code_attribute
class_file = bytes.fromhex('cafebabe') + u2(0) + u2(49)
class_file += u2(len(pool) + 1) + b''.join(pool)
class_file += u2(0x0021) + u2(2) + u2(4) + u2(0) + u2(0)
class_file += u2(1) + main_method + u2(0)
Path(__file__).with_name('Main.class').write_bytes(class_file)

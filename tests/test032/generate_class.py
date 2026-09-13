"""Exercise wide local indexes and signed 16-bit iinc operands."""
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

get_stdout = bytes([0xb2, 0x00, 0x0d])  # getstatic System.out
println = bytes([0xb6, 0x00, 0x13])     # invokevirtual println(int)
wide_load = bytes([0xc4, 0x15, 0x01, 0x00])  # wide iload 256
code = bytes([
    0x04,                                   # iconst_1
    0xc4, 0x36, 0x01, 0x00,                 # wide istore 256
])
code += get_stdout + wide_load + println     # 1
code += bytes([
    0xc4, 0x84, 0x01, 0x00, 0x01, 0x2c,     # wide iinc 256, 300
])
code += get_stdout + wide_load + println     # 301
code += bytes([
    0xc4, 0x84, 0x01, 0x00, 0xfe, 0xd2,     # wide iinc 256, -302
])
code += get_stdout + wide_load + println     # -1
code += bytes([0xb1])                        # return

code_attribute = u2(2) + u2(257) + u4(len(code)) + code + u2(0) + u2(0)
main_method = u2(0x0009) + u2(5) + u2(6) + u2(1)
main_method += u2(7) + u4(len(code_attribute)) + code_attribute
class_file = bytes.fromhex('cafebabe') + u2(0) + u2(49)
class_file += u2(len(pool) + 1) + b''.join(pool)
class_file += u2(0x0021) + u2(2) + u2(4) + u2(0) + u2(0)
class_file += u2(1) + main_method + u2(0)
Path(__file__).with_name('Main.class').write_bytes(class_file)

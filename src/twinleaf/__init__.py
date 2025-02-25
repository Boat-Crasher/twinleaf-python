import twinleaf._twinleaf
import struct
from types import SimpleNamespace

class Device(_twinleaf.Device):
    def __new__(cls, url=None, route=None):
        return super().__new__(cls, url, route)

    def __init__(self, url=None, route=None):
        super().__init__()

    def _rpc_int(self, name: str, size: int, signed: bool, value: int | None = None) -> int:
        payload = b'' if value is None else value.to_bytes(size, signed=signed)
        rep = self.rpc(name, payload)
        return int.from_bytes(rep, signed=signed)

    def _rpc_float(self, name: str, size: int, value: float | None = None) -> float:
        fstr = '<f' if (size == 4) else '<d'
        payload = b'' if value is None else struct.pack(fstr, value)
        rep = self.rpc(name, payload)
        return struct.unpack(fstr, rep)[0]

    def _get_method(self, name: str, meta: int):
        data_type = (meta & 0xF)
        data_size = (meta >> 4) & 0xF
        if (meta & 0x8000) == 0:
            def method(arg: bytes = b'') -> bytes:
                return self.rpc(name, arg)
        elif data_size == 0:
            def method() -> None:
                return self.rpc(name, b'')
        elif data_type in (0, 1):
            signed = (data_type) == 1
            if (meta & 0x0200) == 0:
                def method() -> int:
                    return self._rpc_int(name, data_size, signed)
            else:
                def method(arg: int | None = None) -> int:
                    return self._rpc_int(name, data_size, signed, arg)
        elif data_type == 2:
            if (meta & 0x0200) == 0:
                def method() -> float:
                    return self._rpc_float(name, data_size)
            else:
                def method(arg: float | None = None) -> float:
                    return self._rpc_float(name, data_size, arg)
        elif data_type == 3:
            if (meta & 0x0200) == 0:
                def method() -> str:
                    return self.rpc(name, b'').decode()
            else:
                def method(arg: str | None = None) -> str:
                    return self.rpc(name, arg.encode()).decode()
        return method

    def scan_rpcs(self):
        n = int.from_bytes(self.rpc("rpc.listinfo", b""), "little")
        for i in range(n):
            res = self.rpc("rpc.listinfo", i.to_bytes(2, "little"))
            meta = int.from_bytes(res[0:2], "little")
            name = res[2:].decode()

            mname, *prefix = reversed(name.split("."))
            parent = self
            if prefix and (prefix[-1] == "rpc"):
                prefix[-1] = "_rpc"
            for token in reversed(prefix):
                if not hasattr(parent, token):
                    setattr(parent, token, SimpleNamespace())
                parent = getattr(parent, token)

            setattr(parent, mname, self._get_method(name, meta))


__doc__ = twinleaf.__doc__
if hasattr(twinleaf, "__all__"):
    __all__ = twinleaf.__all__

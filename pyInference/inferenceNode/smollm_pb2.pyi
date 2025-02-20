from google.protobuf import descriptor as _descriptor
from google.protobuf import message as _message
from typing import ClassVar as _ClassVar, Optional as _Optional

DESCRIPTOR: _descriptor.FileDescriptor

class SmolLMReq(_message.Message):
    __slots__ = ("prompt",)
    PROMPT_FIELD_NUMBER: _ClassVar[int]
    prompt: str
    def __init__(self, prompt: _Optional[str] = ...) -> None: ...

class SmolLMRes(_message.Message):
    __slots__ = ("curToken", "tokenNum", "tokenProbability")
    CURTOKEN_FIELD_NUMBER: _ClassVar[int]
    TOKENNUM_FIELD_NUMBER: _ClassVar[int]
    TOKENPROBABILITY_FIELD_NUMBER: _ClassVar[int]
    curToken: str
    tokenNum: int
    tokenProbability: float
    def __init__(self, curToken: _Optional[str] = ..., tokenNum: _Optional[int] = ..., tokenProbability: _Optional[float] = ...) -> None: ...

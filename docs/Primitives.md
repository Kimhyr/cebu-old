# Primitives

```.
Bool        1B
Char        4B
String      B = 4B * A.n

Int8        1B
Int16       2B
Int32       4B
Int64       8B

UInt8       1B
UInt16      2B
UInt32      4B
UInt64      8B

Float32     4B
Float64     8B

Size        >2B
USize       >2B

Array       B = t.S * A.n
Tuple       B = x: S; ∀i∈T, x += i.S
```
# Syntax

## Variables
```rust
U8 var_name = 12
I128 var_name_but_signed = 12998234
F32 var_name_but_floating = 13.9
STRING var_name_but_a_string = "a string"
```

## IfElse

```rust
IF __boolean_label {
    __true_block_label
} ELSE {
    __false_block_label
}
```

## Loops

```rust
LOOP __lo_bound_label __hi_bound_label {
    __loop_body_label
}
```

## Critical

```rust
CRITICAL {
    __crit_block_label
}
```

## Function calls

```rust
CALL __function_label
```

## Function declarations

``` rust
FUNC __function_label {
    __stmt_label_0
    __stmt_label_1
    __stmt_label_2
    __stmt_label_3
    RET __stmt_label_4
}
```

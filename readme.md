toy_rust_db 
---

A little database + basic sql-like query language

```
select code, avg from test where avg > 50
select code, avg from test where dept == "MATH"
select code, avg from test where avg > 50 and dept == "MATH"
```
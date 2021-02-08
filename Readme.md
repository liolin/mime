# Mime
Mime is a blacksmith from the Hero sage.

You give mime the task to build your model and mime generates it for you. Like a blacksmith.
[Wikipedia - Mime](https://de.wikipedia.org/wiki/Mime_(Schmied))

## Example
```
[user@pc]$ mime generate model model id:i32 name:String:pub secret:String:pri
Generate the following model:

struct Model {
    pub id: i32,
    pub name: String,
    secret: String,
}
```

```
[user@pc]$ bat -p src/models/model.rs
struct Model {
    pub id: i32,
    pub name: String,
    secret: String,
}
```

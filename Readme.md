# Mime
Mime is a blacksmith from the Hero sage.

You give mime the task to build your model and mime generates it for you. Like a blacksmith.
[Wikipedia - Mime](https://de.wikipedia.org/wiki/Mime_(Schmied))

## Example
```
mime generate model model id:i32 name:String:pub
```

```
Generate the following model:

struct Model {
    pub id: i32,
    pub name: String,
}
```
```
bat -p src/models/model.rs
```
```
struct Model {
    pub id: i32,
    pub name: String,
}
```

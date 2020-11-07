# Websocket

## Patterns

```typescript
interface Request<T> {
    type: "req",
    id: string,
    method: string,
    params: T
}
```

```typescript
interface Response<T> {
    type: "res",
    id: string,
    result: T,
}
```

```typescript
interface Error {
    type: "err",
    id?: string,
    eid: string,
    message: string,
}
```

A _Notification_ is a _Request_ without `id`, i.e. it does not expect a response. 

```typescript
interface Notification<T> {
    type: "not",
    id: string,
    method: string,
    params: T
}
```
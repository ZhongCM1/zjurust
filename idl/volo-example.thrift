namespace rs volo.example

enum RedisCommand {
    Get,
    Set,
    Del,
    Ping,
}

struct GetItemRequest {
    1: required RedisCommand cmd,
    2: optional string key,
    3: optional string value,
}

struct GetItemResponse {
    1: required bool flag,
    2: required string res,
}

service ItemService {
    GetItemResponse GetItem (1: GetItemRequest req),
}

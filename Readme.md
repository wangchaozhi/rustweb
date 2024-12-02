#添加序列化支持
   如果需要自动为生成的实体结构派生 serde 的 Serialize 和 Deserialize 特性，你需要显式指定 --with-serde 参数。它支持以下选项：

serialize：仅生成 Serialize 特性。
deserialize：仅生成 Deserialize 特性。
both：同时生成 Serialize 和 Deserialize 特性。


sea-orm-cli generate entity -o src/models --with-serde both
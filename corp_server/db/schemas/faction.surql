DEFINE TABLE faction SCHEMALESS;

DEFINE FIELD name ON faction TYPE string ASSERT $value != NONE;

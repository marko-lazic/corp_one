DEFINE TABLE colony SCHEMALESS;

DEFINE FIELD name ON colony TYPE string ASSERT $value != NONE;

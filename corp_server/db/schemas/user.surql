DEFINE TABLE user SCHEMALESS;

DEFINE FIELD username ON user TYPE string ASSERT $value != NONE;
DEFINE FIELD email ON user TYPE string ASSERT string::is::email($value);
DEFINE FIELD password ON user TYPE string ASSERT $value != NONE;
DEFINE FIELD registered_at ON user TYPE datetime DEFAULT time::now() VALUE time::now();

DEFINE INDEX email ON TABLE user COLUMNS email UNIQUE;

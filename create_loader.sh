EXTENSION_NAME=$( find . -iname "*.wit" -exec basename {} .wit ';')
WASM_B64=$(base64 -w 0 "${EXTENSION_NAME}".wasm)
WIT_B64=$(base64 -w 0 "${EXTENSION_NAME}".wit)

CMD="CREATE OR REPLACE AGGREGATE bloom_filter(text CHARACTER SET utf8mb4 COLLATE utf8mb4_general_ci NOT NULL)
RETURNS LONGBLOB NOT NULL
WITH STATE HANDLE
AS WASM FROM BASE64 '$WASM_B64'
WITH WIT FROM BASE64 '$WIT_B64'
INITIALIZE WITH bloom_init_handle
ITERATE WITH bloom_update_handle
MERGE WITH bloom_merge_handle
TERMINATE WITH bloom_serialize_handle
SERIALIZE WITH bloom_serialize_handle
DESERIALIZE WITH bloom_deserialize_handle;\n"

VALUE_FUNCTIONS=$(grep 'func(' "${EXTENSION_NAME}".wit | grep -v handle | sed -E -e 's/([\w]*):.*/\1/g' | sed 's/-/_/g')
for func in $VALUE_FUNCTIONS
do
  CMD="$CMD CREATE OR REPLACE FUNCTION $func AS WASM FROM BASE64 '$WASM_B64' WITH WIT FROM BASE64 '$WIT_B64';\n"
done

echo "$CMD" > load_extension.sql

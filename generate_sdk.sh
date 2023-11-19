docker run --rm \
  -v ${PWD}/open_api_generated:/local openapitools/openapi-generator-cli generate \
  -i "https://stoplight.io/api/v1/projects/spacetraders/spacetraders/nodes/reference/SpaceTraders.json?fromExportButton=true&snapshotType=http_service&deref=optimizedBundle" \
  -g rust \
  -o /local \
  --additional-properties=preferUnsignedInt=true --additional-properties=supportMultipleResponses=false \
  --skip-validate-spec

# openapi-generator-cli generate -i source/SpaceTraders.json -g rust --skip-validate-spec --additional-properties=preferUnsignedInt=true --additional-properties=supportMultipleResponses=true